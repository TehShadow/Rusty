use axum::{extract::{State, Json, Path}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::middleware::USER;


#[derive(Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
}

#[derive(Deserialize)]
pub struct PostMessageRequest {
    pub content: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub sender_id: String,
    pub content: String,
    pub created_at: String,
}



pub async fn post_message_to_room(
    State(db): State<PgPool>,
    Path(room_id): Path<Uuid>,
    Json(payload): Json<PostMessageRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_id = USER
        .with(|user| Uuid::parse_str(&user.user_id))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        r#"INSERT INTO messages (room_id, sender_id, content) VALUES ($1, $2, $3)"#,
        room_id,
        user_id,
        payload.content
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}

pub async fn get_messages_by_room(
    State(db): State<PgPool>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<Vec<MessageResponse>>, StatusCode> {
    let raw_msgs = sqlx::query!(
        r#"
        SELECT id, sender_id, content, created_at
        FROM messages
        WHERE room_id = $1
        ORDER BY created_at ASC
        "#,
        room_id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let messages = raw_msgs
        .into_iter()
        .map(|m| MessageResponse {
            id: m.id.to_string(),
            sender_id: m.sender_id.to_string(),
            content: m.content,
            created_at: m.created_at.to_string(),
        })
        .collect();

    Ok(Json(messages))
}
