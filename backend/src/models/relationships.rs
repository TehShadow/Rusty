use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Serialize ,Deserialize)]
pub struct Relationship {
    pub user_id: Uuid,
    pub related_user_id: Uuid,
    pub status: String,
    pub created_at : OffsetDateTime
}