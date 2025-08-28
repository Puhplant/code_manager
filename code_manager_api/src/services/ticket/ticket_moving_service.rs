use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::tickets::move_ticket};

use crate::{
    models::api_error::ApiError,
    controllers::validation::ticket_validation::MoveTicketRequest,
    services::helpers::get_connection,
};

#[async_trait]
pub trait ITicketMovingService: Send + Sync {
    async fn move_ticket(&self, request: &MoveTicketRequest, ticket_id: i32, account_id: i32) -> Result<(), ApiError>;
}

pub struct TicketMovingService {
    pub pool: Pool,
}

#[async_trait]
impl ITicketMovingService for TicketMovingService {
    async fn move_ticket(&self, request: &MoveTicketRequest, ticket_id: i32, account_id: i32) -> Result<(), ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let rows_affected = move_ticket()
            .bind(&client, &request.column_id, &request.position, &ticket_id, &account_id)
            .await
            .map_err(|e| {
                println!("Error moving ticket: {:?}", e);
                ApiError::InternalServerError
            })?;

        if rows_affected == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
} 