use axum::{extract::State, Json};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{controllers::validation::auth_validation::{LoginValidation, RegisterValidation}, middleware::validation::Validatable, models::api_error::ApiError, registration::container::Container};

#[axum::debug_handler]
pub async fn login(
    State(state): State<Container>,
    jar: CookieJar,
    login_request: Validatable<LoginValidation>,
) -> Result<(CookieJar, Json<LoginResponse>), ApiError> {
    let login_request = login_request.validate()?;
    let login_service = state.create_scope().get_login_service();
    let login_response = login_service.login(&login_request.email, &login_request.password).await?;

    let mut cookie = Cookie::new("refresh_token", format!("{}:{}", login_response.refresh_token.id, login_response.refresh_token.token));
    //cookie.set_expires(Expiration::DateTime(login_response.refresh_token.expires_at));
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    let jar = jar.add(cookie);
    
    Ok((jar, Json(LoginResponse {
        token: login_response.token,
        user_id: login_response.user_id,
        expires_at: login_response.expires_at,
    })))
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
    user_id: i32,
    expires_at: DateTime<Utc>,
}

#[axum::debug_handler]
pub async fn register(
    State(state): State<Container>,
    register_request: Validatable<RegisterValidation>,
) -> Result<(), ApiError> {
    let register_request = register_request.validate()?;
    let registration_service = state.create_scope().get_registration_service();
    registration_service.register(&register_request.name, &register_request.email, &register_request.password, register_request.account_id).await?;

    Ok(())
}

#[axum::debug_handler]
pub async fn refresh_token(
    State(state): State<Container>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<RefreshTokenResponse>), ApiError> {
    // Extract refresh token from cookie
    let refresh_token_cookie = jar.get("refresh_token")
        .ok_or(ApiError::Unauthorized)?;
    
    let cookie_value = refresh_token_cookie.value();
    let parts: Vec<&str> = cookie_value.split(':').collect();
    
    if parts.len() != 2 {
        println!("Invalid refresh token cookie");
        return Err(ApiError::Unauthorized);
    }
    
    let refresh_token_id = parts[0].parse::<Uuid>()
        .map_err(|_| {
            println!("Invalid refresh token cookie");
            ApiError::Unauthorized
        })?;
    
    let refresh_service = state.create_scope().get_refresh_token_service();
    let refresh_response = refresh_service.refresh_token(refresh_token_id).await?;

    let mut cookie = Cookie::new("refresh_token", format!("{}:{}", refresh_response.id, refresh_response.token));
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_same_site(SameSite::Strict);
    let jar = jar.add(cookie);
    
    Ok((jar, Json(RefreshTokenResponse {
        token: refresh_response.token,
        user_id: refresh_response.user_id,
        expires_at: refresh_response.expires_at,
    })))
}

#[derive(Serialize)]
pub struct RefreshTokenResponse {
    token: String,
    user_id: i32,
    expires_at: DateTime<Utc>,
}
