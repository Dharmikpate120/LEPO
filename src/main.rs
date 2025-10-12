
use axum::{
    body::Body,
    extract::{ Multipart, Query, State },
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Form,
    Json,
    Router,

};
use gemini_client_rs::{
    types::{
        Content,
        ContentPart,
        FunctionCallingConfig,
        FunctionCallingMode,
        GenerateContentRequest,
        Role,
        Tool,
        ToolConfig,
        ToolConfigFunctionDeclaration,
    },
    GeminiClient,
};
use axum_macros::debug_handler;
use tokio::net::TcpListener;
use serde::{ Deserialize };
use dotenvy::dotenv;
use std::{ env, fmt::Debug };
use serde_json::json;
use sqlx::PgPool;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

use anyhow::Context;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();

    let config = lepo::config::Config::parse();

    println!("{:?}",config);

    let db = sqlx::postgres::PgPoolOptions::new().max_connections(50).connect(&config.database_uri).await.context("could not connect to the database!")?;

    // sqlx::migrate!().run(&db).await?;

    lepo::http::serve(config,db).await?;

    Ok(())
}