use axum::{
    extract::{Path, State,Extension},
    Json,
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::SimpleUser;
use crate::models::messages::{DirectMessage,SendMessageInput};
use crate::auth::middleware::CurrentUser;


pub async fn get_user_by_id(
    Path(user_id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<SimpleUser>, (StatusCode, String)> {
    let user = sqlx::query_as!(
        SimpleUser,
        r#"
        SELECT id, username
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}

pub async fn send_direct_message(
    Path(receiver_id): Path<Uuid>,
    Extension(CurrentUser { id: sender_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
    Json(payload): Json<SendMessageInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if sender_id == receiver_id {
        return Err((StatusCode::BAD_REQUEST, "Cannot message yourself".into()));
    }

    sqlx::query!(
        r#"
        INSERT INTO direct_messages (sender_id, receiver_id, content)
        VALUES ($1, $2, $3)
        "#,
        sender_id,
        receiver_id,
        payload.content
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({ "result": "message_sent" })))
}

pub async fn get_direct_messages(
    Path(other_user_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<DirectMessage>>, (StatusCode, String)> {
    if my_id == other_user_id {
        return Err((StatusCode::BAD_REQUEST, "Cannot load DMs with yourself".into()));
    }

    let messages = sqlx::query_as!(
        DirectMessage,
        r#"
        SELECT id, sender_id, receiver_id, content, created_at
        FROM direct_messages
        WHERE
            (sender_id = $1 AND receiver_id = $2)
            OR
            (sender_id = $2 AND receiver_id = $1)
        ORDER BY created_at ASC
        "#,
        my_id,
        other_user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(messages))
}