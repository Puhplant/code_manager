use clorinde::deadpool_postgres::{Client, Pool};

use crate::models::api_error::ApiError;

pub async fn get_connection(pool: &Pool) -> Result<Client, ApiError> {
    pool.get().await.map_err(|e| {
        println!("Error getting client: {}", e);
        ApiError::InternalServerError
    })
}