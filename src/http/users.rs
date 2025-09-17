use axum::Router;
use axum::routing::get;

use crate::http::extractor::AuthUser;

pub fn router() ->Router{
    Router::new().route("/users", get(get_handler))
}

async fn get_handler(auth_user:AuthUser) -> &'static str{
    "hello world"
}