use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct LoginServiceResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user_id: i32,
    pub refresh_token: RefreshToken,
}

pub struct RefreshToken {
    pub id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}