use gemini_client_rs::{GeminiClient, GeminiError, types::{Content, ContentData, ContentPart, FunctionCall, GenerateContentRequest, Role, Tool, ToolConfigFunctionDeclaration}};
use reqwest::Client;
use serde::Deserialize;

use crate::http::{AppContext, gemini::{function_calls::{add_call, multiply_call}, function_declarations::{add_declaration, multiply_declaration}, instructions::se_instruction}, types::{commitsresponse::{CommitsResponse, FirstCommitsResponse}, githubcommits::GithubCommits}, users::BodyStruct};

#[derive(Debug, Deserialize)]
struct TwoIntegerArguments{
    number1: u64,
    number2: u64

}
pub fn filter_commit_content(commit_content: String) -> String {
    let filtered = commit_content.split("diff --git");
    let mut output = "".to_string();
    for file in filtered{
        // if file.contains("dist/"){
        //     println!("contains dist");
        // }
        if 
        !file.contains("package-lock.json") && 
        !file.contains("/build/") && 
        !file.contains("/dist/") && 
        !file.contains(".next/") && 
        !file.contains("/public") && 
        !file.contains("/assets") && 
        !file.contains(".svg") {
            // println!("filtered: {}", &file);
             output += "diff --git";
             output += file 
        }
    }
    output
}
pub async fn skills_extractor(user_commits:Vec<GithubCommits>,body:BodyStruct, ctx:&AppContext)->Result<(FirstCommitsResponse,Vec<CommitsResponse>), Box<dyn std::error::Error>>{

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
    
    let mut skills:Vec<CommitsResponse> = vec![];
    let mut skills_list:FirstCommitsResponse = FirstCommitsResponse{technologies: vec![]};

    for (index, commit) in &mut user_commits.iter().rev().enumerate(){

        //fetching commit content
        let  commit_content = Client::new()
        .get(format!("https://api.github.com/repos/{}/{}/commits/{}", &body.user_name, &body.repo_name, commit.sha))
        .header("Accept", "application/vnd.github.v3.diff")
        .header("User-Agent","Lepo/1.0")
        .send().await?
        .text().await?;

        let filtered_content = filter_commit_content(commit_content);

        println!("{:?}",filtered_content);
        contents.push( Content {
                parts: vec![ContentPart {
                    thought: false,
                    data: ContentData::Text(filtered_content),
                    metadata: None,
                }],
                role: Role::User,
            });

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
                if index == 0 {
                    let parsed_args: Result<FirstCommitsResponse, serde_json::Error> = serde_json::from_str(&json_str);
                    // let values;
                    match parsed_args{
                        Ok(data)=>{
                            // values =data;
                            contents.pop();
                            contents.push(response.candidates[0].content.clone());
                            skills_list = data; 
                        }
                        Err(err)=>{
                            println!("error while parsing response arguments!: {:#?}", err);
                            return Err(Box::new(err));
                        }
                    }
                }else{
                    let parsed_args: Result<CommitsResponse, serde_json::Error> = serde_json::from_str(&json_str);
                    let values;
                    match parsed_args{
                        Ok(data)=>{
                            values =data;
                            contents.pop();
                            contents.push(response.candidates[0].content.clone());
                            skills.push(values); 
                        }
                        Err(err)=>{
                            println!("error while parsing response arguments!: {:#?}", err);
                            return Err(Box::new(err));
                        }
                    }
                }
                
            }
            
            _ =>{
                println!("different type of response!")
            }
            
        }
    }   
    Ok((skills_list,skills))
}