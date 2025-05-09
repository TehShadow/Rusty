use axum::{extract::{Extension, State , Path}, Json};
use sqlx::PgPool;
use crate::auth::middleware::CurrentUser;
use crate::models::relationships::Relationship;
use uuid::Uuid;
use http::StatusCode;
use serde_json::json;



pub async fn send_friend_request(
    Path(target_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
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
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "result": "request_sent" })))
}


pub async fn accept_friend_request(
    Path(other_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
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
    .execute(&pool)
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
    State(pool): State<PgPool>,
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
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "result": "blocked" })))
}

pub async fn list_friends(
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Relationship>>, (StatusCode, String)> {
    let friends = sqlx::query_as!(
        Relationship,
        r#"
        SELECT user_id, related_user_id, status, created_at
        FROM user_relationships
        WHERE status = 'friends' AND (user_id = $1 OR related_user_id = $1)
        "#,
        my_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(friends))
}

pub async fn list_pending_requests(
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Relationship>>, (StatusCode, String)> {
    let pending = sqlx::query_as!(
        Relationship,
        r#"
        SELECT user_id, related_user_id, status, created_at
        FROM user_relationships
        WHERE status = 'pending' AND (user_id = $1 OR related_user_id = $1)
        "#,
        my_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(pending))
}

pub async fn remove_relationship(
    Path(other_id): Path<Uuid>,
    Extension(CurrentUser { id: my_id, .. }): Extension<CurrentUser>,
    State(pool): State<PgPool>,
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
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "No relationship to remove".into()));
    }

    Ok(Json(serde_json::json!({ "result": "removed" })))
}