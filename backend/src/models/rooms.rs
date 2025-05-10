use uuid::Uuid;
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct CreateRoomInput {
    pub name: String,
}

#[derive(Serialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: OffsetDateTime,
}

#[derive(Deserialize)]
pub struct RoomMessageInput {
    pub content: String,
}

#[derive(Serialize , Deserialize)]
pub struct RoomMessage {
    pub id: Uuid,
    pub room_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub edited_at: Option<OffsetDateTime>,
}

#[derive(Serialize , Deserialize)]
pub struct RoomInfo {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub created_at: OffsetDateTime,
}