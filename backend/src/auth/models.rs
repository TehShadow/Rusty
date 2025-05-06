use serde::{Deserialize, Serialize};
use time::OffsetDateTime;


#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub created_at: Option<OffsetDateTime>
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: Option<String>,
    pub sender_id: Option<String>,
    pub recipient_id: Option<String>,
    pub content: String,
    pub sent_at: Option<OffsetDateTime>,
}

