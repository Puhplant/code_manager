use std::sync::Arc;

use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2};
use async_trait::async_trait;

use crate::{models::api_error::{ApiError, BadRequestError}, services::auth::{user_creator::IUserCreator, user_email_getter::IUserEmailGetter}};


#[async_trait]
pub trait IRegistrationService: Send + Sync {
    async fn register(&self, name: &str, email: &str, password: &str, account_id: i32) -> Result<(), ApiError>;
}

pub struct RegistrationService {
    pub user_email_getter: Arc<dyn IUserEmailGetter>,
    pub user_creator: Arc<dyn IUserCreator>,
}

#[async_trait]
impl IRegistrationService for RegistrationService {
    async fn register(&self, name: &str, email: &str, password: &str, account_id: i32) -> Result<(), ApiError> {
        let user = self.user_email_getter.get_user_email(&email).await?;

        if user.is_some() {
            return Err(ApiError::BadRequest(BadRequestError {
                message: "User already exists".to_string(),
                field_errors: None,
            }));
        }

        let salt = SaltString::generate(OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|e| {
            println!("Error hashing password: {}", e);
            ApiError::InternalServerError
        })?.to_string();

        self.user_creator.create_user(name, email, &password_hash, account_id).await?;

        Ok(())
    }
}