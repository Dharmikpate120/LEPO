
use axum::{Extension, Json, Router};
use axum::routing::{post};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::http::gemini::functions::skills_extractor;
use crate::http::types::commitsresponse::{CommitsResponse, FirstCommitsResponse};
use crate::http::types::githubcommits::GithubCommits;
use crate::http::{AppContext, Result};
use chrono::{DateTime, Utc};


#[derive(Deserialize, Debug)]
pub struct BodyStruct{
    pub access_token:String,
    pub repo_name:String,
    pub user_name: String,
    pub user_email: String
    
}

#[derive(serde::Serialize)]
pub struct GetSkillsreturn{
    pub skills: Vec<String>
}

#[derive(Deserialize, Serialize)]
pub struct CommitDiff{
    pub commit: String
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    summary:String,
    username:String,
    repo_name:String,
    summary_id:Uuid
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoredSummary{
    pub initial_commit: FirstCommitsResponse,
    pub all_commits: Vec<CommitsResponse>
}

pub fn router() ->Router{
    Router::new().route("/git/calculate-skills", post(get_handler))
}

#[axum::debug_handler]
async fn get_handler( ctx:Extension<AppContext>, Json(body):Json<BodyStruct>) ->  Result<Json<(FirstCommitsResponse,Vec<CommitsResponse>)>>{

    //fetching currently stored user's details
    let user= sqlx::query_as!(User,"SELECT commit_summary.summary, commit_summary.username, commit_summary.repo_name, commit_summary.summary_id FROM commit_summary INNER JOIN users ON commit_summary.user_id = users.user_id WHERE repo_name = $1 AND commit_summary.username=$2;
    ",
    &body.repo_name,
    &body.user_name
    ).fetch_one(&ctx.db).await?;

    let stored_summary:StoredSummary = serde_json::from_str(&user.summary)?;


    //getting all the commits from the github
    let client = Client::new();
    let response = client.get(format!("https://api.github.com/repos/{}/{}/commits", &body.user_name, &body.repo_name)).header("User-Agent","Lepo/1.0").send().await?.text().await?;
    // println!("{}",response);
    let user_commits :Vec<GithubCommits> = serde_json::from_str(&response)?;

    //gemini calling function to extract skills
    let result = skills_extractor(stored_summary,user_commits,&body, &ctx).await;

    //extracting Commitsresponse from result
    let skills_list: FirstCommitsResponse ;
    // skills_list = FirstCommitsResponse{
    //     commit_id:"".to_string(),
    //     commit_message:"".to_string(),
    //     technologies: vec![]
    // };
    let skills: Vec<CommitsResponse>;
    // skills = vec![];
    match result{
        Ok(sk)=>{
            skills_list = sk.0;
            skills = sk.1;
        }
        Err(err) => {
            println!("{:?}",err);
            return Err(crate::http::error::Error::RequestFailed);
        }
    }

    
    // println!("{:?}", serde_json::to_string(
    //     &StoredSummary{
    //     initial_commit: FirstCommitsResponse {
    //         commit_id: "()".to_string(),
    //         commit_message: "()".to_string(), 
    //         technologies: vec![] 
    //     },
    //     all_commits: vec![]
    // }));
    // println!("{:#?}, {}",user_commits, user_commits.len());
    // let mut commit_vec: Vec<CommitDiff> = vec![];
    
    // println!("{:#?}",skills);

    

    // println!("accesstoken:{}", body.access_token);
    Ok(Json((skills_list,skills)))

}