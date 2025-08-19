use std::sync::Arc;

use async_trait::async_trait;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Duration, Utc};

use crate::{models::{api_error::{ApiError, BadRequestError}, services::login_response::LoginServiceResponse}, services::auth::{jwt_auth::JwtAuth, refresh_token_creator::IRefreshTokenCreator, user_email_getter::IUserEmailGetter}};

#[async_trait]
pub trait ILoginService: Send + Sync {
    async fn login(&self, email: &str, password: &str) -> Result<LoginServiceResponse, ApiError>;
}

pub struct LoginService {
    pub jwt_auth: Arc<JwtAuth>,
    pub user_email_getter: Arc<dyn IUserEmailGetter>,
    pub refresh_token_creator: Arc<dyn IRefreshTokenCreator>,
}

#[async_trait]
impl ILoginService for LoginService {
    async fn login(&self, email: &str, password: &str) -> Result<LoginServiceResponse, ApiError> {
        let user = self.user_email_getter.get_user_email(email).await?.ok_or(ApiError::BadRequest(BadRequestError {
            message: "The email address or password is incorrect".to_string(),
            field_errors: None,
        }))?;

        let parsed_password = PasswordHash::new(&user.password_hash)
            .map_err(|e| {
                println!("Error parsing password hash: {}", e);
                ApiError::BadRequest(BadRequestError {
                    message: "The email address or password is incorrect".to_string(),
                    field_errors: None,
                })
            })?;

        if let Err(err) = Argon2::default().verify_password(&password.as_bytes(), &parsed_password) {
            println!("Invalid password: {}", err);
            return Err(ApiError::BadRequest(BadRequestError {
                message: "The email address or password is incorrect".to_string(),
                field_errors: None,
            }));
        }

        let token = self.jwt_auth.create_token(&user.id.to_string(), user.account_id).map_err(|e| {
            println!("Error creating token: {}", e);
            ApiError::InternalServerError
        })?;

        let refresh_token = self.refresh_token_creator.create_refresh_token(user.id).await?;

        Ok(LoginServiceResponse {
            token: token,
            user_id: user.id,
            refresh_token: refresh_token,
            expires_at: Utc::now() + Duration::minutes(30),
        })
    }
}