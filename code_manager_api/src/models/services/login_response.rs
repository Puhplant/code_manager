use chrono::{DateTime, Utc};

pub struct LoginServiceResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user_id: i32,
    pub refresh_token: RefreshToken,
}

pub struct RefreshToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}