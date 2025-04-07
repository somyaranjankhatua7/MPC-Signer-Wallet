use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub error: Option<String>,
}
pub enum AxumApiResponse<T> {
    SUCCESS(StatusCode, JsonApiResponse<T>),
    ERROR(StatusCode, JsonApiResponse<T>)
}

impl<T> IntoResponse for AxumApiResponse<T>
where
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SUCCESS(success, body) => (success, Json(body)).into_response(),
            Self::ERROR(error, body) => (error, Json(body)).into_response()
        }
    }
}