use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::tickets::edit_ticket};

use crate::{
    models::api_error::ApiError,
    controllers::validation::ticket_validation::EditTicketRequest,
    services::helpers::get_connection,
};

#[async_trait]
pub trait ITicketEditingService: Send + Sync {
    async fn edit_ticket(&self, request: &EditTicketRequest, ticket_id: i32, account_id: i32) -> Result<(), ApiError>;
}

pub struct TicketEditingService {
    pub pool: Pool,
}

#[async_trait]
impl ITicketEditingService for TicketEditingService {
    async fn edit_ticket(&self, request: &EditTicketRequest, ticket_id: i32, account_id: i32) -> Result<(), ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let rows_affected = edit_ticket()
            .bind(&client, &request.title, &request.description, &request.column_id, &ticket_id, &account_id)
            .await
            .map_err(|e| {
                println!("Error editing ticket: {:?}", e);
                ApiError::InternalServerError
            })?;

        if rows_affected == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(())
    }
} 