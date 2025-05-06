use axum::{extract::State, Json};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Serialize)]
pub struct SimpleUser {
    id: Option<String>,
    username: Option<String>,
}

pub async fn get_users(State(db): State<PgPool>) -> Json<Vec<SimpleUser>> {
    let users = sqlx::query_as!(
        SimpleUser,
        "SELECT id::text, username FROM users"
    )
    .fetch_all(&db)
    .await
    .unwrap_or_default();

    Json(users)
}
