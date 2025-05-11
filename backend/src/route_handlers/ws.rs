use axum::{
    extract::{Path, State, WebSocketUpgrade, Query},
    http::StatusCode,
    response::IntoResponse,
    extract::ws::{Message, WebSocket},
};
use tokio::sync::broadcast;
use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use uuid::Uuid;
use crate::AppState;
use axum::debug_handler;
use std::sync::Arc;
use serde_json::json;
use std::collections::HashMap;

#[debug_handler]
pub async fn ws_handler(
    Path(room_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(t) => t.clone(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, state, room_id, token))
}

#[debug_handler]
pub async fn ws_dm_handler(
    Path(other_user_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
    State(state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let token = match params.get("token") {
        Some(t) => t.clone(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    ws.on_upgrade(move |socket| handle_dm_socket(socket, state, other_user_id, token))
}



pub async fn handle_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    room_id: Uuid,
    token: String,
) {
    // Decode and verify JWT
    let claims = match crate::auth::jwt::decode_jwt(&token) {
        Ok(c) => c,
        Err(_) => return,
    };

    let user_id = match Uuid::parse_str(&claims.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return,
    };

    // Get or create broadcast sender for the room
    let tx = {
        let mut rooms = state.rooms.write().await;
        rooms.entry(room_id)
            .or_insert_with(|| broadcast::channel::<String>(100).0) // buffer size: 100
            .clone()
    };

    // Subscribe to the broadcast channel
    let mut rx = tx.subscribe();

    // Split the socket into send and receive halves
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to send messages from broadcast receiver to this WebSocket
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Main receive loop from the client
    while let Some(Ok(Message::Text(content))) = receiver.next().await {
        let content_str = content.to_string();

        // Save message to the database
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

        // Broadcast to the room (including sender)
        let _ = tx.send(
            serde_json::to_string(&json!({
                "author_id": user_id,
                "content": content_str,
                "created_at": chrono::Utc::now(),
            })).unwrap()
        );
    }
}

pub async fn handle_dm_socket(
    socket: WebSocket,
    state: Arc<AppState>,
    other_user_id: Uuid,
    token: String,
) {
    // Decode and verify JWT
    let claims = match crate::auth::jwt::decode_jwt(&token) {
        Ok(c) => c,
        Err(_) => return,
    };

    let user_id = match Uuid::parse_str(&claims.claims.sub) {
        Ok(uuid) => uuid,
        Err(_) => return,
    };

    // Create a unique key for the DM pair (sorted to avoid duplication)
    let (a, b) = if user_id < other_user_id {
        (user_id, other_user_id)
    } else {
        (other_user_id, user_id)
    };
    let dm_key = format!("{a}_{b}");

    // Get or create broadcast sender for this DM session
    let tx = {
        let mut rooms = state.rooms.write().await;
        rooms
            .entry(Uuid::new_v5(&Uuid::NAMESPACE_OID, dm_key.as_bytes()))
            .or_insert_with(|| broadcast::channel::<String>(100).0)
            .clone()
    };

    // Subscribe to the broadcast channel
    let mut rx = tx.subscribe();

    // Split the socket into send and receive halves
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to forward broadcasted messages to this socket
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Receive loop
    while let Some(Ok(Message::Text(content))) = receiver.next().await {
        let content_str = content.to_string();

        // Save to direct_messages table
        let _ = sqlx::query!(
            r#"
            INSERT INTO direct_messages (sender_id, receiver_id, content)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            other_user_id,
            content_str
        )
        .execute(&state.pool)
        .await;

        // Broadcast the message as JSON
        let _ = tx.send(
            serde_json::to_string(&json!({
                "sender_id": user_id,
                "content": content_str,
                "created_at": chrono::Utc::now(),
            })).unwrap()
        );
    }
}