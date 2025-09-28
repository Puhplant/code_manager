use async_trait::async_trait;
use clorinde::{deadpool_postgres::Pool, queries::tickets::{get_board_tickets_by_board_id, get_backlog_tickets_by_board_id, MinTicket}};
use std::{collections::HashMap, sync::Arc};
use serde::Serialize;

use crate::{
    models::api_error::ApiError,
    services::helpers::get_connection,
    services::board::board_column_service::{IBoardColumnService, ColumnResponse},
};

#[derive(Debug, Clone, Serialize)]
pub struct MinTicketResponse {
    pub id: i32,
    pub title: String,
    pub column_id: Option<i32>,
    pub position: Option<f64>,
    pub account_id: i32,
    pub user_id: i32,
}

impl From<MinTicket> for MinTicketResponse {
    fn from(ticket: MinTicket) -> Self {
        Self {
            id: ticket.id,
            title: ticket.title,
            column_id: ticket.column_id,
            position: ticket.position,
            account_id: ticket.account_id,
            user_id: ticket.user_id,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ColumnTickets {
    pub column: ColumnResponse,
    pub tickets: Vec<MinTicketResponse>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoardTicketsResponse {
    pub columns: Vec<ColumnTickets>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacklogTicketsResponse {
    pub tickets: Vec<MinTicketResponse>,
}

#[async_trait]
pub trait ITicketBoardService: Send + Sync {
    async fn get_board_tickets(&self, board_id: i32, account_id: i32) -> Result<BoardTicketsResponse, ApiError>;
    async fn get_backlog_tickets(&self, board_id: i32, account_id: i32) -> Result<BacklogTicketsResponse, ApiError>;
}

pub struct TicketBoardService {
    pub pool: Pool,
    pub board_column_service: Arc<dyn IBoardColumnService>,
}

#[async_trait]
impl ITicketBoardService for TicketBoardService {
    async fn get_board_tickets(&self, board_id: i32, account_id: i32) -> Result<BoardTicketsResponse, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        // Get columns for the board
        let board_columns = self.board_column_service.get_columns_by_board_id(board_id, account_id).await?;
        
        // Create a map of column_id to column info
        let column_map: HashMap<i32, ColumnResponse> = board_columns
            .into_iter()
            .map(|col| (col.id, col))
            .collect();
        
        let tickets = get_board_tickets_by_board_id()
            .bind(&client, &board_id, &account_id)
            .all().await
            .map_err(|e| {
                println!("Error getting board tickets: {:?}", e);
                ApiError::InternalServerError
            })?;

        // Group tickets by column_id
        let mut tickets_by_column: HashMap<i32, Vec<MinTicketResponse>> = HashMap::new();
        for ticket in tickets {
            if let Some(column_id) = ticket.column_id {
                tickets_by_column.entry(column_id).or_insert_with(Vec::new).push(MinTicketResponse::from(ticket));
            }
        }

        // Convert to sorted vector of ColumnTickets
        // Include all columns, even those without tickets
        let mut columns: Vec<ColumnTickets> = column_map
            .into_iter()
            .map(|(column_id, column)| {
                let mut tickets = tickets_by_column
                    .get(&column_id)
                    .cloned()
                    .unwrap_or_default();
                
                // Sort tickets by position within each column
                tickets.sort_by(|a, b| {
                    a.position.unwrap_or(0.0).partial_cmp(&b.position.unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal)
                });
                
                ColumnTickets { 
                    column, 
                    tickets 
                }
            })
            .collect();

        // Sort columns by column id
        columns.sort_by_key(|col| col.column.id);

        Ok(BoardTicketsResponse { columns })
    }

    async fn get_backlog_tickets(&self, board_id: i32, account_id: i32) -> Result<BacklogTicketsResponse, ApiError> {
        let client = get_connection(&self.pool).await?;
        
        let tickets = get_backlog_tickets_by_board_id()
            .bind(&client, &board_id, &account_id)
            .all().await
            .map_err(|e| {
                println!("Error getting backlog tickets: {:?}", e);
                ApiError::InternalServerError
            })?;

        let ticket_responses: Vec<MinTicketResponse> = tickets.into_iter().map(MinTicketResponse::from).collect();

        Ok(BacklogTicketsResponse { tickets: ticket_responses })
    }
} 