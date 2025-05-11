use axum::{
    extract::{Path, State,Extension},
    Json,
    http::StatusCode,
};
use uuid::Uuid;
use serde_json::json;
use crate::auth::middleware::CurrentUser;
use crate::models::rooms::{CreateRoomInput,Room,RoomMessage,RoomMessageInput , RoomInfo , Member};


use crate::state::AppState;
use std::sync::Arc;

pub async fn get_room(
    Path(room_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<RoomInfo>, (StatusCode, String)> {
    let room = sqlx::query_as!(
        RoomInfo,
        r#"
        SELECT id, name, owner_id, created_at
        FROM rooms
        WHERE id = $1
        "#,
        room_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok(Json(room))
}


pub async fn create_room(
    Extension(CurrentUser { id: owner_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRoomInput>,
) -> Result<Json<Room>, (StatusCode, String)> {
    let room = sqlx::query_as!(
        Room,
        r#"
        INSERT INTO rooms (name, owner_id)
        VALUES ($1, $2)
        RETURNING id, name, owner_id, created_at
        "#,
        payload.name,
        owner_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // auto-join the creator
    sqlx::query!(
        r#"
        INSERT INTO room_members (user_id, room_id)
        VALUES ($1, $2)
        "#,
        owner_id,
        room.id
    )
    .execute(&state.pool)
    .await
    .ok();

    Ok(Json(room))
}

pub async fn join_room(
    Path(room_id): Path<Uuid>,
    Extension(CurrentUser { id: user_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    sqlx::query!(
        r#"
        INSERT INTO room_members (user_id, room_id)
        VALUES ($1, $2)
        ON CONFLICT DO NOTHING
        "#,
        user_id,
        room_id
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "result": "joined" })))
}

pub async fn list_my_rooms(
    Extension(CurrentUser { id: user_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Room>>, (StatusCode, String)> {
    let rooms = sqlx::query_as!(
        Room,
        r#"
        SELECT r.id, r.name, r.owner_id, r.created_at
        FROM rooms r
        JOIN room_members m ON r.id = m.room_id
        WHERE m.user_id = $1
        "#,
        user_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(rooms))
}

pub async fn send_room_message(
    Path(room_id): Path<Uuid>,
    Extension(CurrentUser { id: author_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RoomMessageInput>,
) -> Result<Json<RoomMessage>, (StatusCode, String)> {
    let message = sqlx::query_as!(
        RoomMessage,
        r#"
        INSERT INTO messages (room_id, author_id, content)
        VALUES ($1, $2, $3)
        RETURNING id, room_id, author_id, content, created_at, edited_at
        "#,
        room_id,
        author_id,
        payload.content
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(message))
}


pub async fn get_room_messages(
    Path(room_id): Path<Uuid>,
    Extension(CurrentUser { id: user_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<RoomMessage>>, (StatusCode, String)> {
    // Optional: verify user is in the room
    let authorized = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM room_members
            WHERE user_id = $1 AND room_id = $2
        ) AS "exists!"
        "#,
        user_id,
        room_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Authorization check failed".to_string()))?;

    if !authorized {
        return Err((StatusCode::UNAUTHORIZED, "You are not in this room".to_string()));
    }

    let messages = sqlx::query_as!(
        RoomMessage,
        r#"
        SELECT id, room_id, author_id, content, created_at, edited_at
        FROM messages
        WHERE room_id = $1
        ORDER BY created_at ASC
        "#,
        room_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(messages))
}

pub async fn list_room_members(
    Path(room_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Member>>, (StatusCode, String)> {
    let members = sqlx::query_as!(
        Member,
        r#"
        SELECT u.id, u.username
        FROM room_members rm
        JOIN users u ON rm.user_id = u.id
        WHERE rm.room_id = $1
        "#,
        room_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(members))
}