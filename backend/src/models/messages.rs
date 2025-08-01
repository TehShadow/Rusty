use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct SendMessageInput {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct DirectMessage {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
    #[serde(serialize_with = "time::serde::rfc3339::serialize")]
    pub created_at: OffsetDateTime
}
