use axum::{http::StatusCode, response::{IntoResponse, Response}};

#[derive(thiserror::Error, Debug)]
pub enum Error{

}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // You can customize the status code and body based on the error type
        (StatusCode::INTERNAL_SERVER_ERROR, format!("HTTP Error: {}", self)).into_response()
    }
}