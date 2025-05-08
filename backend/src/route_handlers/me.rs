use axum::{
    extract::State,
    http::StatusCode, Extension, Json
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::auth::current_user::CurrentUser;
use crate::auth::models::UserProfile;

#[axum::debug_handler]
pub async fn me_handler(
    State(db): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>
) -> Result<Json<UserProfile>, StatusCode> {
    let user_uuid = Uuid::parse_str(&current_user.user_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let row = sqlx::query!(
        r#"
        SELECT id, username, created_at
        FROM users
        WHERE id = $1
        "#,
        user_uuid
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