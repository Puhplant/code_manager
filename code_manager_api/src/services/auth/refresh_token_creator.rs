use async_trait::async_trait;
use chrono::*;
use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2};
use uuid::Uuid;
use clorinde::{deadpool_postgres::Pool, queries::auth::{delete_expired_refresh_tokens, insert_refresh_token}};
use crate::models::{api_error::ApiError, services::login_response::RefreshToken};
use crate::services::helpers::get_connection;

#[async_trait]
pub trait IRefreshTokenCreator: Send + Sync {
    async fn create_refresh_token(&self, user_id: i32) -> Result<RefreshToken, ApiError>;
}

pub struct RefreshTokenCreator {
    pub pool: Pool,
}

#[async_trait]
impl IRefreshTokenCreator for RefreshTokenCreator {
    async fn create_refresh_token(&self, user_id: i32) -> Result<RefreshToken, ApiError> {
        let (token, token_hash) = create_refresh_token(30)?;

        let client = get_connection(&self.pool).await?;

        _ = insert_refresh_token().bind(&client, &token.id, &user_id, &token_hash, &token.expires_at.naive_utc()).one().await.map_err(|e| {
            println!("Error creating refresh token: {}", e);
            ApiError::InternalServerError
        })?;

        _ = delete_expired_refresh_tokens().bind(&client, &user_id).await.map_err(|e| {
            println!("Error expiring refresh token: {}", e);
            ApiError::InternalServerError
        });

        Ok(token)   
    }
}

pub fn create_refresh_token(duration_hours: u32) -> Result<(RefreshToken, String), ApiError> {
    let token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(duration_hours as i64);

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let token_hash = argon2.hash_password(token.as_bytes(), &salt).map_err(|e| {
        println!("Error hashing token: {}", e);
        ApiError::InternalServerError
    })?.to_string();

    Ok((RefreshToken { token, expires_at, id: Uuid::new_v4() }, token_hash))
}