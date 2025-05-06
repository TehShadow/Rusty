use axum::{extract::{State, Json}, http::StatusCode};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::auth::middleware::USER;

#[derive(Deserialize)]
pub struct AddFriendRequest {
    pub friend_id: String,
}

#[derive(Serialize)]
pub struct FriendResponse {
    pub id: Option<String>,
    pub username: Option<String>,
}

pub async fn list_friends(
    State(db): State<PgPool>,
) -> Result<Json<Vec<FriendResponse>>, StatusCode> {
    let user_id = USER.with(|user| user.user_id.clone());
    let user_uuid = Uuid::parse_str(&user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let friends = sqlx::query_as!(
        FriendResponse,
        r#"
        SELECT u.id::TEXT, u.username
        FROM friendships f
        JOIN users u ON f.friend_id = u.id
        WHERE f.user_id = $1
        "#,
        user_uuid
    )
    .fetch_all(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(friends))
}

pub async fn add_friend(
    State(db): State<PgPool>,
    Json(payload): Json<AddFriendRequest>,
) -> Result<StatusCode, StatusCode> {

    let user_id = USER.with(|user| user.user_id.clone());
    let user_uuid = Uuid::parse_str(&user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let friend_uuid = Uuid::parse_str(&payload.friend_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    sqlx::query!(
        r#"
        INSERT INTO friendships (user_id, friend_id)
        VALUES ($1, $2) ON CONFLICT DO NOTHING
        "#,
        user_uuid,
        friend_uuid
    )
    .execute(&db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
