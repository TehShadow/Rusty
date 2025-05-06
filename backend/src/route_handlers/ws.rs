use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use serde_json::json;
use sqlx::PgPool;
use std::{collections::HashMap, sync::Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::auth::jwt::decode_jwt;

static ROOMS: Lazy<Mutex<HashMap<Uuid, broadcast::Sender<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn ws_handler(
    Path(room_id): Path<Uuid>,
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    State(db): State<PgPool>,
) -> impl IntoResponse {
    // Extract token from query (?token=...)
    let token = params.get("token").cloned();

    let user_id = token
        .and_then(|t| {
            let t = t.strip_prefix("Bearer ").unwrap_or(&t);
            decode_jwt(t).ok()
        })
        .map(|claims| claims.sub)
        .and_then(|id| Uuid::parse_str(&id).ok());

    let user_id = match user_id {
        Some(uid) => uid,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Set up broadcast channel for the room
    let tx = {
        let mut rooms = ROOMS.lock().unwrap();
        rooms.entry(room_id)
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    ws.on_upgrade(move |socket| handle_socket(socket, room_id, user_id, db, tx))
}

async fn handle_socket(
    socket: WebSocket,
    room_id: Uuid,
    user_id: Uuid,
    db: PgPool,
    tx: broadcast::Sender<String>,
) {
    let mut rx = tx.subscribe();
    let (mut sender, mut receiver) = socket.split();

    // Sending messages to client
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Receiving messages from client
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt_bytes))) = receiver.next().await {
            let content = txt_bytes.to_string();

            // Fetch sender's username
            if let Ok(sender_row) = sqlx::query!(
                "SELECT username FROM users WHERE id = $1",
                user_id
            )
            .fetch_one(&db)
            .await
            {
                // Save message to DB
                let _ = sqlx::query!(
                    "INSERT INTO messages (room_id, sender_id, content) VALUES ($1, $2, $3)",
                    room_id,
                    user_id,
                    content
                )
                .execute(&db)
                .await;

                // Prepare broadcast payload
                let payload = json!({
                    "sender_id": user_id.to_string(),
                    "username": sender_row.username,
                    "content": content
                });

                let _ = tx.send(payload.to_string());
            }
        }
    });

    let _ = tokio::join!(send_task, recv_task);
}
