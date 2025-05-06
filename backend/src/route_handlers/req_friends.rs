// src/routes/friends.rs
use axum::{extract::{Path, State}, Json};
use uuid::Uuid;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct FriendRequestPayload {
    pub friend_id: Uuid,
}

#[derive(Serialize)]
pub struct FriendRequestRow {
    pub id: String,
    pub sender_id: String,
    pub sender_username: String,
    pub created_at: String,
}

pub async fn add_friend_request(
    State(db): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<FriendRequestPayload>,
) -> Result<StatusCode, StatusCode> {
    let friend_id = payload.friend_id;
    if friend_id == user_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    let already_friends = sqlx::query_scalar!(
        "SELECT 1 FROM friendships WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)",
        user_id, friend_id
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if already_friends.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    sqlx::query!(
        "INSERT INTO friend_requests (sender_id, receiver_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        user_id,
        friend_id
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_friend_requests(
    State(db): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<FriendRequestRow>>, StatusCode> {
    let rows = sqlx::query!(
        r#"
        SELECT fr.id, fr.sender_id, u.username as sender_username, fr.created_at
        FROM friend_requests fr
        JOIN users u ON u.id = fr.sender_id
        WHERE fr.receiver_id = $1
        ORDER BY fr.created_at ASC
        "#,
        user_id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let requests = rows
        .into_iter()
        .map(|r| FriendRequestRow {
            id: r.id.to_string(),
            sender_id: r.sender_id.to_string(),
            sender_username: r.sender_username,
            created_at: r.created_at.to_string(),
        })
        .collect();

    Ok(Json(requests))
}

pub async fn accept_friend_request(
    State(db): State<PgPool>,
    Path((user_id, request_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let row = sqlx::query!(
        "DELETE FROM friend_requests WHERE id = $1 AND receiver_id = $2 RETURNING sender_id",
        request_id,
        user_id
    )
    .fetch_optional(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(r) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    let (a, b) = if user_id < r.sender_id {
        (user_id, r.sender_id)
    } else {
        (r.sender_id, user_id)
    };

    sqlx::query!(
        "INSERT INTO friendships (user_id, friend_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        a, b
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_friend_request(
    State(db): State<PgPool>,
    Path((user_id, request_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!(
        "DELETE FROM friend_requests WHERE id = $1 AND receiver_id = $2",
        request_id,
        user_id
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct FriendResponse {
    pub id: Option<String>,
    pub username: Option<String>,
}

pub async fn list_friends(
    State(db): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Vec<FriendResponse>>, StatusCode> {
    let rows = sqlx::query!(
        r#"
        SELECT u.id, u.username
        FROM friendships f
        JOIN users u ON u.id = CASE
            WHEN f.user_id = $1 THEN f.friend_id
            ELSE f.user_id
        END
        WHERE f.user_id = $1 OR f.friend_id = $1
        "#,
        user_id
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let friends = rows
        .into_iter()
        .map(|row| FriendResponse {
            id: Some(row.id.to_string()),
            username: Some(row.username),
        })
        .collect();

    Ok(Json(friends))
}