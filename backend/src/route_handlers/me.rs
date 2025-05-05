use axum::{extract::State, Json, http::StatusCode};
use crate::auth::middleware::USER;
use crate::auth::models::UserProfile;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn me_handler(State(db): State<PgPool>) -> Result<Json<UserProfile>, StatusCode> {
    let user = USER.with(|u| u.clone());
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = sqlx::query!(
        r#"
        SELECT id, username, created_at
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(&db)
    .await
    .map_err(|e| {
        eprintln!("DB error in /me: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match result {
        Some(row) => Ok(Json(UserProfile {
            id: row.id.to_string(),
            username: row.username,
            created_at: row.created_at,
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}
