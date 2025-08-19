use axum::{extract::{State, Path}, Extension, Json};
use serde::Serialize;

use crate::{
    controllers::validation::ticket_validation::{CreateTicketValidation, GetTicketsByBoardIdValidation, EditTicketValidation},
    middleware::{validation::Validatable},
    models::{api_error::ApiError, user_details::UserDetails},
    registration::container::Container,
    services::ticket::ticket_board_service::{BoardTicketsResponse, BacklogTicketsResponse},
};

#[axum::debug_handler]
pub async fn create_ticket(
    State(state): State<Container>,
    user_details: Extension<UserDetails>,
    create_ticket_request: Validatable<CreateTicketValidation>,
) -> Result<Json<CreateTicketResponse>, ApiError> {
    let create_ticket_request = create_ticket_request.validate()?;
    let ticket_service = state.create_scope().get_ticket_service();
    
    let ticket_id = ticket_service.create_ticket(
        &create_ticket_request,
        user_details.account_id,
        user_details.id,
    ).await?;

    Ok(Json(CreateTicketResponse { id: ticket_id }))
}

#[axum::debug_handler]
pub async fn get_ticket_by_id(
    State(state): State<Container>,
    user_details: Extension<UserDetails>,
    Path(ticket_id): Path<i32>,
) -> Result<Json<GetTicketResponse>, ApiError> {
    let ticket_retrieval_service = state.create_scope().get_ticket_retrieval_service();
    
    let ticket = ticket_retrieval_service.get_ticket_by_id(
        ticket_id,
        user_details.account_id,
    ).await?;

    match ticket {
        Some(ticket) => Ok(Json(GetTicketResponse {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description,
            column_id: ticket.column_id,
            position: ticket.position,
            account_id: ticket.account_id,
            user_id: ticket.user_id,
        })),
        None => Err(ApiError::NotFound),
    }
}

#[axum::debug_handler]
pub async fn get_tickets_by_board_id(
    State(state): State<Container>,
    user_details: Extension<UserDetails>,
    create_ticket_request: Validatable<GetTicketsByBoardIdValidation>,
) -> Result<Json<GetTicketsByBoardIdResponse>, ApiError> {
    let request = create_ticket_request.validate()?;
    let ticket_board_service = state.create_scope().get_ticket_board_service();
    
    match request.ticket_type.as_str() {
        "board" => {
            let board_tickets = ticket_board_service.get_board_tickets(
                request.board_id,
                user_details.account_id,
            ).await?;
            
            Ok(Json(GetTicketsByBoardIdResponse::Board(board_tickets)))
        },
        "backlog" => {
            let backlog_tickets = ticket_board_service.get_backlog_tickets(
                request.board_id,
                user_details.account_id,
            ).await?;
            
            Ok(Json(GetTicketsByBoardIdResponse::Backlog(backlog_tickets)))
        },
        _ => Err(ApiError::BadRequest(crate::models::api_error::BadRequestError {
            message: "Invalid ticket type".to_string(),
            field_errors: None,
        })),
    }
}

#[axum::debug_handler]
pub async fn edit_ticket(
    State(state): State<Container>,
    user_details: Extension<UserDetails>,
    Path(ticket_id): Path<i32>,
    edit_ticket_request: Validatable<EditTicketValidation>,
) -> Result<(), ApiError> {
    let edit_ticket_request = edit_ticket_request.validate()?;
    let ticket_editing_service = state.create_scope().get_ticket_editing_service();
    
    ticket_editing_service.edit_ticket(
        &edit_ticket_request,
        ticket_id,
        user_details.account_id,
    ).await?;

    Ok(())
}

#[derive(Serialize)]
pub struct CreateTicketResponse {
    id: i32,
}

#[derive(Serialize)]
pub struct GetTicketResponse {
    id: i32,
    title: String,
    description: String,
    column_id: Option<i32>,
    position: Option<f64>,
    account_id: i32,
    user_id: i32,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum GetTicketsByBoardIdResponse {
    Board(BoardTicketsResponse),
    Backlog(BacklogTicketsResponse),
}
