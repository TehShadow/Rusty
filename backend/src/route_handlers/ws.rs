use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::{HeaderMap,StatusCode},
    response::IntoResponse,
    extract::ws::{Message, WebSocket},
};
use uuid::Uuid;
use crate::AppState;
use axum::debug_handler;
use std::sync::Arc;

#[debug_handler]
pub async fn ws_handler(
    Path(room_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {

    let token = match extract_token_from_headers(&headers) {
        Some(t) => t,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id, token))
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    room_id: Uuid,
    token: String,
) {
    let claims = match crate::auth::jwt::decode_jwt(&token) {
        Ok(c) => c,
        Err(_) => return,
    };

    let user_id = match Uuid::parse_str(&claims.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return,
    };

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(content) = msg {
            let content_str = content.as_str();

            let _ = sqlx::query!(
                r#"
                INSERT INTO messages (room_id, author_id, content)
                VALUES ($1, $2, $3)
                "#,
                room_id,
                user_id,
                content_str
            )
            .execute(&state.pool)
            .await;

            // Send response or broadcast if needed
        }
    }
}
