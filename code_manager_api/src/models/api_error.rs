use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}};
use serde_json;
use serde::Serialize;

pub enum ApiError {
    InternalServerError,
    NotFound,
    Unauthorized,
    BadRequest(BadRequestError),
}

#[derive(Serialize)]
pub struct FieldError {
    pub field: &'static str,
    pub message: String,
}

#[derive(Serialize)]
pub struct BadRequestError {
    pub message: String,
    pub field_errors: Option<Vec<FieldError>>,
}


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::InternalServerError => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from("Internal server error")).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
            ApiError::NotFound => Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("Not found")).unwrap_or(StatusCode::NOT_FOUND.into_response()),
            ApiError::Unauthorized => Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("Unauthorized")).unwrap_or(StatusCode::UNAUTHORIZED.into_response()),
            ApiError::BadRequest(error) => {
                let body = serde_json::to_string(&error).unwrap_or("Bad request".to_string());
                Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from(body)).unwrap_or(StatusCode::BAD_REQUEST.into_response())
            }
        }
    }
}