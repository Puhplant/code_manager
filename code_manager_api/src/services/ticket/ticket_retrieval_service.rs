use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::tickets::{get_ticket_by_id, Ticket}};

use crate::{
    models::api_error::ApiError,
    services::helpers::get_connection,
};

#[async_trait]
pub trait ITicketRetrievalService: Send + Sync {
    async fn get_ticket_by_id(&self, ticket_id: i32, account_id: i32) -> Result<Option<Ticket>, ApiError>;
}

pub struct TicketRetrievalService {
    pub pool: Pool,
}

#[async_trait]
impl ITicketRetrievalService for TicketRetrievalService {
    async fn get_ticket_by_id(&self, ticket_id: i32, account_id: i32) -> Result<Option<Ticket>, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let ticket = get_ticket_by_id()
            .bind(&client, &ticket_id, &account_id)
            .opt().await
            .map_err(|e| {
                println!("Error getting ticket by id: {:?}", e);
                ApiError::InternalServerError
            })?;

        Ok(ticket)
    }
} 