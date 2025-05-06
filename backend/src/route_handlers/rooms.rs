use axum::{extract::{State, Json, Path}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::middleware::USER;

#[derive(Deserialize)]
pub struct CreateRoomRequest {
    pub name: Option<String>,
    pub is_group: bool,
    pub member_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct RoomResponse {
    pub id: String,
    pub name: Option<String>,
    pub is_group: bool,
}

pub async fn create_room(
    State(db): State<PgPool>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, StatusCode> {
    let user_id = USER
        .with(|user| Uuid::parse_str(&user.user_id))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let room = sqlx::query!(
        r#"INSERT INTO rooms (name, is_group, created_by) VALUES ($1, $2, $3) RETURNING id, name, is_group"#,
        payload.name,
        payload.is_group,
        user_id
    )
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for uid in payload.member_ids {
        let parsed = Uuid::parse_str(&uid).map_err(|_| StatusCode::BAD_REQUEST)?;
        sqlx::query!(
            "INSERT INTO room_members (room_id, user_id) VALUES ($1, $2)",
            room.id,
            parsed
        )
        .execute(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(Json(RoomResponse {
        id: room.id.to_string(),
        name: room.name,
        is_group: room.is_group,
    }))
}

pub async fn list_rooms(
    State(db): State<PgPool>,
) -> Result<Json<Vec<RoomResponse>>, StatusCode> {
    let user_id = USER
        .with(|user| Uuid::parse_str(&user.user_id))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let raw_rooms = sqlx::query!(
        r#"
        SELECT r.id, r.name, r.is_group
        FROM rooms r
        JOIN room_members m ON r.id = m.room_id
        WHERE m.user_id = $1
        "#,
        user_id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rooms = raw_rooms
        .into_iter()
        .map(|r| RoomResponse {
            id: r.id.to_string(),
            name: r.name,
            is_group: r.is_group,
        })
        .collect();

    Ok(Json(rooms))
}

pub async fn list_room_members(
    State(db): State<PgPool>,
    Path(room_id): Path<Uuid>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let members = sqlx::query_scalar!(
        "SELECT user_id::TEXT FROM room_members WHERE room_id = $1",
        room_id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .filter_map(|id| id)
    .collect::<Vec<String>>();

    Ok(Json(members))
}

pub async fn add_room_member(
    State(db): State<PgPool>,
    Path(room_id): Path<Uuid>,
    Json(user): Json<String>,
) -> Result<StatusCode, StatusCode> {
    let user_uuid = Uuid::parse_str(&user).map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query!(
        "INSERT INTO room_members (room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        room_id,
        user_uuid
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}