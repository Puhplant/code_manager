
use async_trait::async_trait;
use clorinde::deadpool_postgres::Pool;
use clorinde::queries::auth::{get_user_by_email, UserPassword};
use crate::{models::api_error::ApiError, services::helpers::get_connection};

#[async_trait]
pub trait IUserEmailGetter: Send + Sync {
    async fn get_user_email(&self, email: &str) -> Result<Option<UserPassword>, ApiError>;
}

pub struct UserEmailGetter {
    pub pool: Pool,
}

#[async_trait]
impl IUserEmailGetter for UserEmailGetter {
    async fn get_user_email(&self, email: &str) -> Result<Option<UserPassword>, ApiError> {
        let normalized_email = email.to_lowercase();
        
        let client = get_connection(&self.pool).await?;
        
        let user = get_user_by_email().bind(&client, &normalized_email).opt().await.map_err(|e| {
            println!("Error getting user by email: {}", e);
            ApiError::InternalServerError
        })?;
        
        match user {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}