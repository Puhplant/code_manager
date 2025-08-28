use crate::{models::{api_error::ApiError, user_details::UserDetails}, registration::container::Container};

use axum::{
    extract::{Request, State}, http, middleware::Next, response::Response
};

const NO_AUTH_PATHS: [&str; 3] = ["/api/auth/login", "/api/auth/refresh", "/api/auth/register"];

pub async fn auth_middleware(State(state): State<Container>, mut req: Request, next: Next) -> Result<Response, ApiError> {
    if NO_AUTH_PATHS.contains(&req.uri().path()) {
        return Ok(next.run(req).await);
    }
    
    let token = req.headers().get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.split_whitespace().nth(1));

    if let Some(token) = token {
        let decoded_token = state.jwt_auth.verify_token(token)
            .map_err(|e| {
                println!("Error verifying token: {:?}", e);
                ApiError::Unauthorized
            })?;

        let user_id = decoded_token.sub.parse::<i32>().map_err(|_| ApiError::Unauthorized)?;
        let account_id = decoded_token.account_id;
        
        req.extensions_mut().insert(UserDetails { id: user_id, account_id: account_id });

        return Ok(next.run(req).await);
    }

    Err(ApiError::Unauthorized)
}

