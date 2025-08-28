use axum::{extract::{FromRequest, Query, Request}, Json};
use serde::de::DeserializeOwned;

use crate::{models::api_error::ApiError, registration::container::Container};

pub trait RequestValidator: DeserializeOwned {
    type RequestOutput;

    fn validate(self) -> Result<Self::RequestOutput, ApiError>;
}

pub struct Validatable<I: RequestValidator>(I);

impl<I: RequestValidator> Validatable<I> {
    pub fn validate(self) -> Result<I::RequestOutput, ApiError> {
        self.0.validate()
    }
}

impl<I: RequestValidator> FromRequest<Container> for Validatable<I> {
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &Container) -> Result<Self, Self::Rejection> {
        let method = req.method();
        
        if method == "GET" {
            let query_result = Query::<I>::from_request(req, state).await;
            match query_result {
                Ok(query) => return Ok(Validatable(query.0)),
                Err(_) => {
                    return Err(ApiError::BadRequest(crate::models::api_error::BadRequestError {
                        message: "Invalid query parameters".to_string(),
                        field_errors: None,
                    }));
                }
            }
        } else {
            let json_result = Json::<I>::from_request(req, state).await;
            match json_result {
                Ok(json) => Ok(Validatable(json.0)),
                Err(_) => Err(ApiError::BadRequest(crate::models::api_error::BadRequestError {
                    message: "Invalid JSON body".to_string(),
                    field_errors: None,
                })),
            }
        }
    }
}
