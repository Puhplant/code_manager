use serde::Deserialize;

use crate::{middleware::validation::RequestValidator, models::api_error::{ApiError, BadRequestError, FieldError}};

pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginValidation {
    email: Option<String>,
    password: Option<String>,
}

impl RequestValidator for LoginValidation {
    type RequestOutput = LoginRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {

            let email = self.email.ok_or(FieldError {
                field: "email",
                message: "Email is required".to_string(),
            }).and_then(|email| {
                if email.is_empty() {
                Err(FieldError {
                    field: "email",
                    message: "Email is required".to_string(),
                })
                } else {
                    Ok(email)
                }
            });

        let password = self.password.ok_or(FieldError {
            field: "password",
            message: "Password is required".to_string(),
        }).and_then(|password| {
            if password.is_empty() {
                Err(FieldError {
                    field: "password",
                    message: "Password is required".to_string(),
                })
            } else {
                Ok(password)
            }
        });

        match (email, password) {
            (Ok(email), Ok(password)) => Ok(LoginRequest { email, password }),
            (email, password) =>{
                let mut validation_errors = Vec::new();
                _ = email.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = password.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
}


pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub account_id: i32,
}

#[derive(Deserialize)]
pub struct RegisterValidation {
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    account_id: Option<i32>,
}

impl RequestValidator for RegisterValidation {
    type RequestOutput = RegisterRequest;

    fn validate(self) -> Result<Self::RequestOutput, ApiError> {
        let name = self.name.ok_or(FieldError {
            field: "name",
            message: "Name is required".to_string(),
        }).and_then(|name| {
            if name.is_empty() {
                Err(FieldError {
                    field: "name",
                    message: "Name is required".to_string(),
                })
            } else {
                Ok(name)
            }
        });

        let email = self.email.ok_or(FieldError {
            field: "email",
            message: "Email is required".to_string(),
        }).and_then(|email| {
            if email.is_empty() {
                Err(FieldError {
                    field: "email",
                    message: "Email is required".to_string(),
                })
            } else {
                Ok(email)
            }
        });

        let password = self.password.ok_or(FieldError {
            field: "password",
            message: "Password is required".to_string(),
        }).and_then(|password| {
            if password.is_empty() {
                Err(FieldError {
                    field: "password",
                    message: "Password is required".to_string(),
                })
            } else {
                Ok(password)
            }
        });

        let account_id = self.account_id.ok_or(FieldError {
            field: "account_id",
            message: "Account ID is required".to_string(),
        }).and_then(|account_id| {
            if account_id == 0 {
                Err(FieldError {
                    field: "account_id",
                    message: "Account ID is required".to_string(),
                })
            } else {
                Ok(account_id)
            }
        });

        match (name, email, password, account_id) {
            (Ok(name), Ok(email), Ok(password), Ok(account_id)) => Ok(RegisterRequest {
                name,
                email,
                password,
                account_id,
            }),
            (name, email, password, account_id) => {
                let mut validation_errors = Vec::new();
                _ =name.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = email.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = password.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                _ = account_id.is_err_and(|e| {
                    validation_errors.push(e);
                    return true;
                });
                Err(ApiError::BadRequest(BadRequestError {
                    message: "Validation errors".to_string(),
                    field_errors: Some(validation_errors),
                }))
            }
        }
    }
}