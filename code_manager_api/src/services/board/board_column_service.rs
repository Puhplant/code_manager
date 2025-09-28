use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::boards::{get_columns_by_board_id, Column}};
use serde::Serialize;

use crate::{
    models::api_error::ApiError,
    services::helpers::get_connection,
};

#[derive(Debug, Clone, Serialize)]
pub struct ColumnResponse {
    pub id: i32,
    pub name: String,
}

impl From<Column> for ColumnResponse {
    fn from(column: Column) -> Self {
        Self {
            id: column.id,
            name: column.name,
        }
    }
}

#[async_trait]
pub trait IBoardColumnService: Send + Sync {
    async fn get_columns_by_board_id(&self, board_id: i32, account_id: i32) -> Result<Vec<ColumnResponse>, ApiError>;
}

pub struct BoardColumnService {
    pub pool: Pool,
}

#[async_trait]
impl IBoardColumnService for BoardColumnService {
    async fn get_columns_by_board_id(&self, board_id: i32, account_id: i32) -> Result<Vec<ColumnResponse>, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let columns = get_columns_by_board_id()
            .bind(&client, &board_id, &account_id)
            .all().await
            .map_err(|e| {
                println!("Error getting columns by board ID: {:?}", e);
                ApiError::InternalServerError
            })?;

        let column_responses: Vec<ColumnResponse> = columns
            .into_iter()
            .map(ColumnResponse::from)
            .collect();

        Ok(column_responses)
    }
}
