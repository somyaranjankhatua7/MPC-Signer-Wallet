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


// #[derive(Serialize, Deserialize, Debug)]
// pub struct SuccessResponse<T> {
//     pub data: Option<T>,
//     pub message: Option<String>,
//     pub status: StatusCode
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ErrorResponse {
//     pub error: Option<String>,
//     pub status: StatusCode
// }


// impl <T> IntoResponse for SuccessResponse<T>
// where 
//     T: Serialize + Debug,
// {
//     fn into_response(self) -> axum::response::Response {
//         let status = self.status;
//         let body = Json(self.data);
//     }
// }