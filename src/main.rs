
// use axum::{
//     body::Body,
//     extract::{ Multipart, Query, State },
//     http::HeaderMap,
//     response::IntoResponse,
//     routing::get,
//     Form,
//     Json,
//     Router,

// };
// use gemini_client_rs::{
//     types::{
//         Content,
//         ContentPart,
//         FunctionCallingConfig,
//         FunctionCallingMode,
//         GenerateContentRequest,
//         Role,
//         Tool,
//         ToolConfig,
//         ToolConfigFunctionDeclaration,
//     },
//     GeminiClient,
// };
// use axum_macros::debug_handler;
// use tokio::net::TcpListener;
// use serde::{ Deserialize };
// use dotenvy::dotenv;
// use std::{ env, fmt::Debug };
// use serde_json::json;
// use sqlx::PgPool;

// #[derive(Clone)]
// struct AppState {
//     pool: PgPool,
// }

use anyhow::Context;
use clap::Parser;
use lepo::http;


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();

    let config = lepo::config::Config::parse();

    println!("{:?}",config);

    let db = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(&config.database_uri).await.context("could not connect to the database!")?;

    sqlx::migrate!().run(&db).await?;

    lepo::http::serve(config,db).await?;

    Ok(())
//     let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL NOT DEFINED!");
//     let pool = sqlx::PgPool::connect(&database_url).await.expect("Failed to create pool.");

//     let app_state: AppState = AppState { pool: pool };
//     let app = Router::new().route(
//         "/v2/webhook",
//         get(validation_handler).post(multipart_tweet_handler).with_state(app_state)
//     );

//     let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// fn wrapper(handler:fn() ->Box) -> Box<dyn IntoResponse> {
//     Box::new("".to_string())
}

// #[derive(Deserialize, Debug)]
// struct Params {
//     #[serde(rename = "hub.challenge")]
//     hub_challenge: Option<String>,
//     #[serde(rename = "hub.verify_token")]
//     hub_verify_token: Option<String>,
// }

// #[derive(Deserialize, Debug)]
// struct BodyType {
//     content: Option<String>,
//     id: Option<String>,
// }

// #[debug_handler]
// async fn validation_handler(Query(params): Query<Params>) -> String {
//     let challenge = params.hub_challenge.unwrap_or("Not provided".to_string());
//     let verify_token = params.hub_verify_token.unwrap_or("Not provided".to_string());
//     format!("challenge: {} \nverify_token: {}", challenge, verify_token)
// }

// #[debug_handler]
// async fn multipart_tweet_handler(
//     State(state): State<AppState>,
//     Query(params): Query<Params>,
//     mut multipart: Multipart
// ) -> String {
//     let mut content = String::new();
//     let mut id;
//     let mut file_content;
//     let mut file;
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap_or("empty").to_string();
//         let filename = field.file_name().unwrap_or("empty").to_string();
//         if name == "content" {
//             content = field.text().await.unwrap_or("empty".to_string()).to_string();
//         } else if name == "id" {
//             id = field.text().await.unwrap_or("empty".to_string()).to_string();
//         } else if filename != "empty" {
//             file_content = field.bytes().await.unwrap_or_default();
//             file = filename;
//         }
//         // println!("name: {},fileName:  {}", name, filename);
//     }

//     let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not found!");
//     let model = env::var("GEMINI_MODEL").expect("GEMINI_MODEL not found!");

//     let client = GeminiClient::new(gemini_api_key);

//     let request_content = GenerateContentRequest {
//         system_instruction: None,
//         contents: vec![Content {
//             parts: vec![
//                 ContentPart::new_text(
//                     "you are an assessor. the following content contains the project details the candidate has provided. your goal is to give the list of technical skills and level of that skill from 100 the perticular candidate contains based only on the following text. in skills make sure you include only purely technical skills don't include general skills such as frontend development, authentication instead include specific skills such as react, nextjs, firebase, mongodb etc. if you find any non technical skills don't include them in the response. if you find any technical skills give them a rating out of 100 based on the level of expertise the candidate has in that skill based on the provided content. also provide a justification for each skill and rating you give. make sure you only include skills that are explicitly mentioned in the content or can be reasonably inferred from it. do not make any assumptions about skills that are not mentioned in the content.",
//                     false
//                 ),
//                 ContentPart::new_text(
//                     "give the response in a strict string format: skill-(skillName),rating-(rating),justification-(justification); ---repeat. here don't return anything else. no extra characters, spaces and nothing just this structured string which can be parsed manually.",
//                     false
//                 ),
//                 ContentPart::new_text(&content, false)
//             ],
//             role: Role::User,
//         }],
//         tools: vec![
//             Tool::GoogleSearch { google_search: json!(null) }
//             // Tool::FunctionDeclaration(ToolConfigFunctionDeclaration {
//             //     function_declarations: vec![],
//             // })
//         ],
//         tool_config: None,
//         // tool_config: Some(ToolConfig {
//         //     function_calling_config: FunctionCallingConfig {
//         //         mode: FunctionCallingMode::Any,
//         //         allowed_function_names: vec![String::from("")],
//         //     },
//         // }),
//         generation_config: None,
//     };

//     // let json_request = json!({
//     //     "contents": request_content
//     // });

//     // let input_content = serde_json::from_value(json_request);

//     // panic!("Gemini API request failed");
//     let response = client.generate_content(&model, &request_content).await;

//     let user_skills: String;

//     println!("params: {:?}", params);
//     // "".to_string()
//     format!("response: {:?}", response)
// }

// async fn tweet_handler(Form(body): Form<BodyType>) -> String {
//     let content = body.content.unwrap_or("No content".to_string());
//     let id = body.id.unwrap_or("No id".to_string());
//     let gemini_api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
//     format!("{}, content: {}, id: {}", &gemini_api_key, &content, &id)

//     //handler tweet hook
// }
