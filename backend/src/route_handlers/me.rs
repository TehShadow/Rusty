use axum::{extract::Extension, Json};
use crate::auth::middleware::CurrentUser;
use serde_json::json;

pub async fn get_me(Extension(user): Extension<CurrentUser>) -> Json<serde_json::Value> {
    Json(json!({
        "id": user.id,
        "username": user.username,
        "session_id": user.session_id,
    }))
}
