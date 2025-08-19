use axum::{extract::{FromRequest, Request}, Json};
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
        let result = <Json<I> as FromRequest<Container>>::from_request(req, state).await;

        match result {
            Ok(validator) => Ok(Validatable(validator.0)),
            Err(_) => Err(ApiError::BadRequest(crate::models::api_error::BadRequestError {
                message: "Invalid JSON".to_string(),
                field_errors: None,
            })),
        }
    }
}
