use axum::{extract::State, Extension, Json};
use serde::Serialize;

use crate::{
    models::{api_error::ApiError, user_details::UserDetails},
    registration::container::Container,
    services::board::board_service::BoardResponse,
};

#[axum::debug_handler]
pub async fn get_boards_by_account_id(
    State(state): State<Container>,
    user_details: Extension<UserDetails>,
) -> Result<Json<GetBoardsByAccountIdResponse>, ApiError> {
    let board_service = state.create_scope().get_board_service();
    
    let boards = board_service.get_boards_by_account_id(user_details.account_id).await?;

    Ok(Json(GetBoardsByAccountIdResponse { boards }))
}

#[derive(Serialize)]
pub struct GetBoardsByAccountIdResponse {
    boards: Vec<BoardResponse>,
} 