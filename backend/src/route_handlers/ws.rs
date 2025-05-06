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

    if let Some(raw) = params.get("token") {
        println!("Raw token: {}", raw);
        let decoded = decode_jwt(raw.strip_prefix("Bearer ").unwrap_or(raw));
        println!("Decoded: {:?}", decoded);
    }
    // Extract token from query string (?token=...)
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

    // Set up broadcast channel
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

    // Task to send messages from broadcast channel to this socket
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Task to receive messages from client and broadcast to others
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt_bytes))) = receiver.next().await {
            let content = txt_bytes.to_string(); // Convert Utf8Bytes -> String

            let payload = json!({
                "sender_id": user_id.to_string(),
                "content": content
            });

            // Insert message into DB
            let _ = sqlx::query!(
                "INSERT INTO messages (room_id, sender_id, content) VALUES ($1, $2, $3)",
                room_id,
                user_id,
                content
            )
            .execute(&db)
            .await;

            // Broadcast message to all subscribers
            let _ = tx.send(payload.to_string());
        }
    });

    // Wait for either task to complete
    let _ = tokio::join!(send_task, recv_task);
}
