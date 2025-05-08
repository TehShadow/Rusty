use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::middleware::USER;

// Response struct
#[derive(Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub created_at: String,
}

pub async fn me_handler(
    State(db): State<PgPool>,
) -> Result<Json<UserProfile>, StatusCode> {
    let user = USER.with(|u| u.clone());
    let user_id = Uuid::parse_str(&user.user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let row = sqlx::query!(
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

    match row {
        Some(user_row) => {
            let profile = UserProfile {
                id: user_row.id.to_string(),
                username: user_row.username,
                created_at: user_row.created_at.to_string(),
            };
            Ok(Json(profile))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
