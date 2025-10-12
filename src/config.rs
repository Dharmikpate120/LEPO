
#[derive(clap::Parser,Debug, Clone)]
pub struct Config{

    #[arg(long, env="DATABASE_URL")]
    pub database_uri:String,

    #[arg(long, env="HMAC_KEY")]
    pub hmac_key: String,

    #[arg(long, env="PORT")]
    pub port:u16,
    #[arg(long, env="HOST")]
    pub host:String,

    #[arg(long, env="GEMINI_API_KEY")]
    pub gemini_api_key : String,

    #[arg(long, env="GEMINI_MODEL")]
    pub gemini_model : String,
}