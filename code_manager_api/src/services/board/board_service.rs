use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::boards::{get_boards_by_account_id, Board}};
use serde::Serialize;

use crate::{
    models::api_error::ApiError,
    services::helpers::get_connection,
};

#[derive(Debug, Clone, Serialize)]
pub struct BoardResponse {
    pub id: i32,
    pub name: String,
}

impl From<Board> for BoardResponse {
    fn from(board: Board) -> Self {
        Self {
            id: board.id,
            name: board.name,
        }
    }
}

#[async_trait]
pub trait IBoardService: Send + Sync {
    async fn get_boards_by_account_id(&self, account_id: i32) -> Result<Vec<BoardResponse>, ApiError>;
}

pub struct BoardService {
    pub pool: Pool,
}

#[async_trait]
impl IBoardService for BoardService {
    async fn get_boards_by_account_id(&self, account_id: i32) -> Result<Vec<BoardResponse>, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let boards = get_boards_by_account_id()
            .bind(&client, &account_id)
            .all().await
            .map_err(|e| {
                println!("Error getting boards by account ID: {:?}", e);
                ApiError::InternalServerError
            })?;

        let board_responses: Vec<BoardResponse> = boards
            .into_iter()
            .map(BoardResponse::from)
            .collect();

        Ok(board_responses)
    }
} 