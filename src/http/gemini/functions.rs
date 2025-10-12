use gemini_client_rs::types::{Content, ContentData, ContentPart, GenerateContentRequest, Role};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::http::{AppContext, gemini::instructions::se_instruction, routes::users::StoredSummary, types::{commitsresponse::{CommitsResponse, FirstCommitsResponse}, githubcommits::GithubCommits}, users::BodyStruct};

#[derive(Debug, Deserialize, Serialize)]
struct InputContent{
    initial_commit: bool,
    commit_id: String,
    commit_message: String,
    commit_diff: String
}



pub fn filter_commit_content(commit_content: String) -> Vec<String> {
    let filtered = commit_content.split("diff --git");
    let mut suboutput = "".to_string();
    for file in filtered{
        // if file.contains("dist/"){
        //     println!("contains dist");
        // }
        if 
        !file.contains("package-lock.json") &&
        !file.contains("node_modules") && 
        !file.contains("Binary files") &&
        !file.contains("similarity index 100%") &&
        !file.contains("README.md") &&
        !file.contains("/build/") && 
        !file.contains("/dist/") && 
        !file.contains(".next/") && 
        !file.contains("/public") && 
        !file.contains("/assets") && 
        !file.contains(".svg") {
            // println!("filtered: {}", &file);
             suboutput += "diff --git";
             suboutput += file 
        }
    }
    let mut output: Vec<String> = vec![];
    let mut temp_file = "".to_string();
    for file in suboutput.split("diff --git"){
        // println!("file:\n{:?}",&file);
        if temp_file.len() + file.len() < 5000{
            temp_file += "diff --git";
            temp_file += file;
        }else{
            output.push(temp_file);
            temp_file = "diff --git".to_string() + file;
        }
    }
    if temp_file !="" {
        output.push(temp_file);
    }
    output
}
pub async fn skills_extractor(
    stored_summary:StoredSummary,
    user_commits:Vec<GithubCommits>,
    body:&BodyStruct, 
    ctx:&AppContext
    )->Result<
        (
            FirstCommitsResponse,
            Vec<CommitsResponse>
        ),
        Box<dyn std::error::Error>>
{
    let mut commitCount = 0;
    if stored_summary.initial_commit.technologies.len()!=0{
        commitCount +=stored_summary.all_commits.len() + 1;
    }
    
    //creating gemini client
    let client = gemini_client_rs::GeminiClient::new(ctx.config.gemini_api_key.clone());

    // contents array to be passed to gemini request
    let mut contents:Vec<Content> = vec![];

    //initial prompt (default)
    contents.push(Content{
            parts:vec![
                ContentPart{
                    thought:false, 
                    data:ContentData::Text(se_instruction()), 
                    metadata:None
                }
            ], 
            role: Role::User
        }
    );
    if commitCount!= 0{

        contents.push(Content{
            parts:vec![
                ContentPart{
                    thought:false, 
                    data:ContentData::Text(
                        serde_json::to_string(
                            &stored_summary.initial_commit
                        )?
                    ), 
                    metadata:None
                }
                ], 
                role: Role::User
            });
        for cr in &stored_summary.all_commits{
            contents.push(Content{
            parts:vec![
                ContentPart{
                    thought:false, 
                    data:ContentData::Text(
                        serde_json::to_string(
                            cr
                        )?
                    ), 
                    metadata:None
                }
                ], 
                role: Role::User
            });
        }   
        }
    let mut skills:Vec<CommitsResponse> = stored_summary.all_commits;
    let mut skills_list:FirstCommitsResponse = stored_summary.initial_commit;
    let mut iterations = 0;
    for (index, commit) in &mut user_commits.iter().rev().enumerate(){
        if index+1 < commitCount || iterations > 1 {
            continue;
        }else {
            iterations += 1;
        }
        //fetching commit content
        let  commit_content = Client::new()
        .get(format!("https://api.github.com/repos/{}/{}/commits/{}", body.user_name, body.repo_name, commit.sha))
        .header("Accept", "application/vnd.github.v3.diff")
        .header("User-Agent","Lepo/1.0")
        .send().await?
        .text().await?;

        let filtered_content = filter_commit_content(commit_content.clone());
        // println!("aq12wsde34rf: {} {:#?}",filtered_content.len(),filtered_content);
        
        // println!("filtered:{}, {}\n", filtered_content.len(), commit_content.len());
        for (inner_index, content) in filtered_content.iter().enumerate()
        {
            if inner_index > 2 {
                break;
            }
            // println!("filteredContent:{}\n{}",inner_index,&content)

            // println!("{:#?}",contents);
            contents.push( Content {
                parts: vec![ContentPart {
                    thought: false,
                    data: ContentData::Text(serde_json::to_string(&InputContent{
                        initial_commit: index == 0,
                        commit_id: commit.sha.to_string(),
                        commit_message: commit.commit.message.to_string(),
                        commit_diff: content.clone()
                    })?),
                    metadata: None,
                }],
                role: Role::User,
            });
            // println!("inputContent: \n{:?}",InputContent{
            //             initial_commit: index == 0,
            //             commit_id: commit.sha.to_string(),
            //             commit_message: commit.commit.message.to_string(),
            //             commit_diff: content.clone()
            //         });
            // console.log()
            let request :GenerateContentRequest = GenerateContentRequest{      
                system_instruction: None, 
                contents: contents.clone(), 
                tools: vec![
                    // Tool::FunctionDeclaration(ToolConfigFunctionDeclaration{
                    //     function_declarations: vec![
                    //         add_declaration(),
                    //         multiply_declaration()
                    //     ]
                    // }) 
                ], 
                tool_config: None, 
                generation_config: None
            };
            
            
            let response = client.generate_content(&ctx.config.gemini_model, &request ).await?;
            
            // map the candidates vector later
            match &response.candidates[0].content.parts[0].data{
                ContentData::Text(txt) =>{
                    let json_str = txt
                        .trim_start_matches("```json\n")
                        .trim_end_matches("\n```");
                    // println!("{:?}",&json_str);
                    // println!("{:?}", &json_str);
                    if index == 0 {
                        
                        let parsed_args: Result<FirstCommitsResponse, serde_json::Error> = serde_json::from_str(&json_str);
                        // let values;
                        match parsed_args{
                            Ok(data)=>{
                                // values =data;
                                contents.pop();
                                if inner_index != 0 {
                                    contents.pop();
                                }
                                contents.push(response.candidates[0].content.clone());
                                skills_list = data; 
                            }
                            Err(err)=>{
                                println!("error while parsing response arguments!1: {:#?}", err);
                                return Ok((skills_list,skills));
                            }

                        }
                    }else{
                        let parsed_args: Result<CommitsResponse, serde_json::Error> = serde_json::from_str(&json_str);
                        let values;
                        match parsed_args{
                            Ok(data)=>{
                                values =data;
                                contents.pop();
                                if inner_index != 0 {
                                    contents.pop();
                                    skills.pop();
                                }
                                contents.push(response.candidates[0].content.clone());
                                skills.push(values); 
                            }
                            Err(err)=>{
                                println!("error while parsing response arguments!2: {:#?}", err);
                                // println!("2")
                                return Ok((skills_list,skills));
                            }
                        }
                    }
                    
                }
                
                _ =>{
                    println!("different type of response!")
                }
                
            }
        }
    }   

    let new_summary = serde_json::to_string(
        &StoredSummary{
        initial_commit:skills_list.clone(),
        all_commits:skills.clone()
    })?;
    let _ = sqlx::query!("UPDATE commit_summary SET summary =$1 WHERE repo_name = $2 AND commit_summary.username=$3;",
    new_summary,
    &body.repo_name,
    &body.user_name
    ).execute(&ctx.db).await?;
    Ok((skills_list,skills))
}