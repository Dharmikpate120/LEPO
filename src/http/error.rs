use axum::{http::StatusCode, response::{IntoResponse, Response}};

#[derive(thiserror::Error, Debug)]
pub enum Error{

    #[error("authentication required")]
    Unauthorized,

    #[error("request to remote server failed!")]
    RequestFailed,
    
    #[error("Failed to parse the response into given Struct type")]
    ParsingFailed
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // You can customize the status code and body based on the error type
        (StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP Error: {}", self)).into_response()
    }
}

impl From<reqwest::Error> for Error{
    fn from(err: reqwest::Error) -> Self {
        println!("reqwest failed: {:#?}", err);
        Error::RequestFailed
    }   
}

impl From<serde_json::Error> for Error{
    fn from(err: serde_json::Error) -> Self{
        println!("Error while trying to parse into json: {:?}", err);
        Error::ParsingFailed
    }
}