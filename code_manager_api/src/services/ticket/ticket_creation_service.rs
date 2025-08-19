use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::tickets::create_ticket};

use crate::{
    models::api_error::ApiError,
    controllers::validation::ticket_validation::CreateTicketRequest,
    services::helpers::get_connection,
};

#[async_trait]
pub trait ITicketCreationService: Send + Sync {
    async fn create_ticket(&self, request: &CreateTicketRequest, account_id: i32, user_id: i32) -> Result<i32, ApiError>;
}

pub struct TicketCreationService {
    pub pool: Pool,
}

#[async_trait]
impl ITicketCreationService for TicketCreationService {
    async fn create_ticket(&self, request: &CreateTicketRequest, account_id: i32, user_id: i32) -> Result<i32, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let id = create_ticket()
            .bind(&client, &request.title, &request.description, &request.column_id, &request.position, &request.board_id, &account_id, &user_id)
            .one().await
            .map_err(|e| {
                println!("Error creating ticket: {:?}", e);
                ApiError::InternalServerError
            })?;

        Ok(id)
    }
} 