use axum::{extract::{State, Json, Path}, http::StatusCode, Router, routing::{get, post}};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Serialize)]
pub struct MessagePayload {
    pub sender_id: String,
    pub username: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct AddRoomMemberRequest {
    user_id: String,
}

#[derive(Serialize)]
pub struct AddRoomMemberResponse {
    status: String,
    room_id: Uuid,
    user_id: Uuid,
}

pub async fn create_room(
    State(db): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<Json<RoomResponse>, StatusCode> {
    if !payload.is_group && payload.member_ids.len() == 1 {
        let other_id = Uuid::parse_str(&payload.member_ids[0])
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let existing = sqlx::query!(
            r#"
            SELECT r.id, u.username
            FROM rooms r
            JOIN room_members rm1 ON rm1.room_id = r.id
            JOIN room_members rm2 ON rm2.room_id = r.id
            JOIN users u ON u.id = rm2.user_id
            WHERE r.is_group = false
              AND ((rm1.user_id = $1 AND rm2.user_id = $2)
                   OR (rm1.user_id = $2 AND rm2.user_id = $1))
              AND rm2.user_id != $1
            LIMIT 1
            "#,
            user_id,
            other_id
        )
        .fetch_optional(&db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if let Some(existing) = existing {
            return Ok(Json(RoomResponse {
                id: existing.id.to_string(),
                name: Some(existing.username),
                is_group: false,
            }));
        }
    }

    let name = if payload.is_group {
        payload.name.clone()
    } else {
        Some("dm".to_string())
    };

    let room = sqlx::query!(
        r#"INSERT INTO rooms (name, is_group, created_by) VALUES ($1, $2, $3) RETURNING id, name, is_group"#,
        name,
        payload.is_group,
        user_id
    )
    .fetch_one(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut all_member_ids = payload.member_ids;
    all_member_ids.push(user_id.to_string());

    for uid in all_member_ids {
        let parsed = Uuid::parse_str(&uid).map_err(|_| StatusCode::BAD_REQUEST)?;
        sqlx::query!(
            "INSERT INTO room_members (room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
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
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<RoomResponse>>, StatusCode> {
    let raw_rooms = sqlx::query!(
        r#"
        SELECT r.id, 
               CASE 
                 WHEN r.is_group THEN r.name 
                 ELSE (
                   SELECT u.username 
                   FROM room_members rm 
                   JOIN users u ON u.id = rm.user_id 
                   WHERE rm.room_id = r.id AND rm.user_id != $1 
                   LIMIT 1
                 ) 
               END as name, 
               r.is_group
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
    Json(payload): Json<AddRoomMemberRequest>,
) -> Result<Json<AddRoomMemberResponse>, StatusCode> {
    let user_uuid = Uuid::parse_str(&payload.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query!(
        "INSERT INTO room_members (room_id, user_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        room_id,
        user_uuid
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(AddRoomMemberResponse {
        status: "member_added".to_string(),
        room_id,
        user_id: user_uuid,
    }))
}