use axum::{extract::State, Json};
use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use chrono::{DateTime, Utc};
use serde::Serialize;

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

    let mut cookie = Cookie::new("refresh_token", login_response.refresh_token.token);
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
