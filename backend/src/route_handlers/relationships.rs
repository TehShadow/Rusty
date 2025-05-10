use axum::{extract::{Extension, State , Path}, Json};
use crate::auth::middleware::CurrentUser;
use crate::models::user::SimpleUser;
use uuid::Uuid;
use http::StatusCode;
use serde_json::json;

use crate::state::AppState;
use std::sync::Arc;

pub async fn send_friend_request(
    Path(target_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if my_id == target_id {
        return Err((StatusCode::BAD_REQUEST, "Can't add yourself".into()));
    }

    let (user_a, user_b) = if my_id < target_id {
        (my_id, target_id)
    } else {
        (target_id, my_id)
    };

    let status = if my_id < target_id { "pending" } else { "pending" };

    sqlx::query!(
        r#"
        INSERT INTO user_relationships (user_id, related_user_id, status)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
        "#,
        user_a,
        user_b,
        status
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "result": "request_sent" })))
}


pub async fn accept_friend_request(
    Path(other_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let (user_a, user_b) = if my_id < other_id {
        (my_id, other_id)
    } else {
        (other_id, my_id)
    };

    let updated = sqlx::query!(
        r#"
        UPDATE user_relationships
        SET status = 'friends'
        WHERE user_id = $1 AND related_user_id = $2 AND status = 'pending'
        "#,
        user_a,
        user_b
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if updated.rows_affected() == 0 {
        return Err((StatusCode::BAD_REQUEST, "No pending request found".into()));
    }

    Ok(Json(json!({ "result": "friends" })))
}

pub async fn block_user(
    Path(other_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let (user_a, user_b) = if my_id < other_id {
        (my_id, other_id)
    } else {
        (other_id, my_id)
    };

    sqlx::query!(
        r#"
        INSERT INTO user_relationships (user_id, related_user_id, status)
        VALUES ($1, $2, 'blocked')
        ON CONFLICT (user_id, related_user_id)
        DO UPDATE SET status = 'blocked'
        "#,
        user_a,
        user_b
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "result": "blocked" })))
}

pub async fn list_friends(
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SimpleUser>>, (StatusCode, String)> {
    let friends = sqlx::query_as!(
        SimpleUser,
        r#"
        SELECT u.id, u.username
        FROM user_relationships ur
        JOIN users u ON
            (u.id = ur.user_id AND ur.related_user_id = $1)
            OR (u.id = ur.related_user_id AND ur.user_id = $1)
        WHERE ur.status = 'friends'
        "#,
        my_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(friends))
}

pub async fn list_pending_requests(
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SimpleUser>>, (StatusCode, String)> {
    let users = sqlx::query_as!(
        SimpleUser,
        r#"
        SELECT u.id, u.username
        FROM user_relationships ur
        JOIN users u ON
            -- this JOIN ensures we get the user who SENT the request
            (u.id = ur.user_id AND ur.related_user_id = $1)
        WHERE ur.status = 'pending'
        "#,
        my_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(users))
}

pub async fn remove_relationship(
    Path(other_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    if my_id == other_id {
        return Err((StatusCode::BAD_REQUEST, "Cannot unfriend yourself".into()));
    }

    let (user_a, user_b) = if my_id < other_id {
        (my_id, other_id)
    } else {
        (other_id, my_id)
    };

    let deleted = sqlx::query!(
        r#"
        DELETE FROM user_relationships
        WHERE user_id = $1 AND related_user_id = $2
        "#,
        user_a,
        user_b
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "No relationship to remove".into()));
    }

    Ok(Json(serde_json::json!({ "result": "removed" })))
}