use async_trait::async_trait;
use chrono::*;
use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Argon2};
use uuid::Uuid;
use clorinde::{deadpool_postgres::Pool, queries::auth::insert_refresh_token};
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

        insert_refresh_token().bind(&client, &user_id, &token_hash, &token.expires_at.naive_utc()).await.map_err(|e| {
            println!("Error creating refresh token: {}", e);
            ApiError::InternalServerError
        })?;

        // sqlx::query(
        //     "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)"
        // )
        // .bind(&user_id)
        // .bind(&token_hash)
        // .bind(&token.expires_at)
        // .execute(&self.pool)
        // .await.map_err(|e| {
        //     println!("Error creating refresh token: {} for user {}", e, user_id);
        //     ApiError::InternalServerError
        // })?;

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

    Ok((RefreshToken { token, expires_at }, token_hash))
}