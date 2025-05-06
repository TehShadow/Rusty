// Full room-based chat handler (rooms.rs)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use time::OffsetDateTime;
use crate::auth::middleware::USER;

#[derive(Deserialize)]
pub struct CreateRoom {
    pub name: Option<String>,
    pub is_group: bool,
    pub member_ids: Vec<String>,
}

#[derive(Deserialize)]
pub struct SendRoomMessage {
    pub content: String,
}

#[derive(Serialize)]
pub struct Room {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
}

#[derive(Serialize)]
pub struct ChatMessage {
    pub id: String,
    pub sender_id: String,
    pub content: String,
    pub sent_at: Option<OffsetDateTime>,
}

pub async fn create_room(
    State(db): State<PgPool>,
    Json(payload): Json<CreateRoom>,
) -> Result<Json<Room>, StatusCode> {
    let room = sqlx::query!(
        "INSERT INTO chat_rooms (name, is_group) VALUES ($1, $2) RETURNING id, name, is_group",
        payload.name,
        payload.is_group
    )
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for user_id in &payload.member_ids {
        let uid = Uuid::parse_str(user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
        sqlx::query!("INSERT INTO room_members (user_id, room_id) VALUES ($1, $2)", uid, room.id)
            .execute(&db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(Room {
        id: room.id.to_string(),
        name: room.name,
        is_group: room.is_group.unwrap_or(false),
    }))
}

pub async fn list_rooms(
    State(db): State<PgPool>,
) -> Result<Json<Vec<Room>>, StatusCode> {
    let user = USER.with(|u| u.clone());
    let uid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let rows = sqlx::query!(
        "SELECT r.id, r.name, r.is_group FROM chat_rooms r \
         JOIN room_members m ON r.id = m.room_id WHERE m.user_id = $1",
        uid
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rooms = rows
        .into_iter()
        .map(|r| Room {
            id: r.id.to_string(),
            name: r.name,
            is_group: r.is_group.unwrap_or(false),
        })
        .collect();

    Ok(Json(rooms))
}

pub async fn send_message_to_room(
    State(db): State<PgPool>,
    Path(room_id): Path<String>,
    Json(payload): Json<SendRoomMessage>,
) -> Result<StatusCode, StatusCode> {
    let user = USER.with(|u| u.clone());
    let uid = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let rid = Uuid::parse_str(&room_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query!(
        "INSERT INTO messages (sender_id, room_id, content) VALUES ($1, $2, $3)",
        uid,
        rid,
        payload.content
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get_messages_by_room(
    State(db): State<PgPool>,
    Path(room_id): Path<String>,
) -> Result<Json<Vec<ChatMessage>>, StatusCode> {
    let rid = Uuid::parse_str(&room_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let msgs = sqlx::query!(
        "SELECT id, sender_id, content, sent_at FROM messages WHERE room_id = $1 ORDER BY sent_at ASC",
        rid
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(
        msgs.into_iter()
            .map(|m| ChatMessage {
                id: m.id.to_string(),
                sender_id: m.sender_id.to_string(),
                content: m.content,
                sent_at: m.sent_at,
            })
            .collect(),
    ))
}