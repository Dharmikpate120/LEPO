
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