
#[derive(clap::Parser,Debug, Clone)]
pub struct Config{

    #[arg(long, env="POSTGRES_URL")]
    pub database_uri:String,

    #[arg(long, env="HMAC_KEY")]
    pub hmac_key: String

}