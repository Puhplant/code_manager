use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;
use clorinde::{deadpool_postgres::Pool, queries::auth::{get_refresh_token_by_id, expire_refresh_token, insert_refresh_token, get_user_by_id}};
use crate::{models::api_error::ApiError, services::{auth::{jwt_auth::JwtAuth, refresh_token_creator::create_refresh_token}, helpers::get_connection}};

#[async_trait]
pub trait IRefreshTokenService: Send + Sync {
    async fn refresh_token(&self, refresh_token_id: Uuid) -> Result<RefreshTokenResponse, ApiError>;
}

pub struct RefreshTokenService {
    pub pool: Pool,
    pub jwt_auth: Arc<JwtAuth>,
}

#[async_trait]
impl IRefreshTokenService for RefreshTokenService {
    async fn refresh_token(&self, refresh_token_id: Uuid) -> Result<RefreshTokenResponse, ApiError> {
        let mut client = get_connection(&self.pool).await?;
        let transaction = client.transaction().await.map_err(|e| {
            println!("Error starting transaction: {}", e);
            ApiError::InternalServerError
        })?;

        // 1. Get the refresh token by ID
        let refresh_token = get_refresh_token_by_id()
            .bind(&transaction, &refresh_token_id)
            .one()
            .await
            .map_err(|e| {
                println!("Error getting refresh token: {}", e);
                ApiError::Unauthorized
            })?;

        // Check if token is expired
        if refresh_token.expires_at < Utc::now().naive_utc() {
            println!("Refresh token is expired");
            return Err(ApiError::Unauthorized);
        }

        // 2. Expire the current refresh token
        expire_refresh_token()
            .bind(&transaction, &refresh_token.user_id, &refresh_token_id)
            .await
            .map_err(|e| {
                println!("Error expiring refresh token: {}", e);
                ApiError::InternalServerError
            })?;

        // 3. Create a new refresh token
        let (new_refresh_token, new_token_hash) = create_refresh_token(30)?;
        
        _ = insert_refresh_token()
            .bind(&transaction, &new_refresh_token.id, &refresh_token.user_id, &new_token_hash, &new_refresh_token.expires_at.naive_utc())
            .one()
            .await
            .map_err(|e| {
                println!("Error creating new refresh token: {}", e);
                ApiError::InternalServerError
            })?;

        // 4. Get the user
        let user = get_user_by_id()
            .bind(&transaction, &refresh_token.user_id)
            .one()
            .await
            .map_err(|e| {
                println!("Error getting user: {}", e);
                ApiError::InternalServerError
            })?;

        // 5. Create a new token
            let (token, expires_at) = self.jwt_auth.create_token(&user.id.to_string(), user.account_id).map_err(|e| {
                println!("Error creating token: {}", e);
                ApiError::InternalServerError
            })?;

        transaction.commit().await.map_err(|e| {
            println!("Error committing transaction: {}", e);
            ApiError::InternalServerError
        })?;

        Ok(RefreshTokenResponse {
            id: new_refresh_token.id,
            token: token,
            expires_at: expires_at,
            user_id: refresh_token.user_id,
        })
    }
}

pub struct RefreshTokenResponse {
    pub id: Uuid,
    pub token: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub user_id: i32,
} 