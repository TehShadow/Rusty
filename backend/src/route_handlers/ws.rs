use axum::{
    extract::{Query, WebSocketUpgrade, State},
    response::IntoResponse,
    extract::ws::{WebSocket, Message},
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;
use crate::auth::jwt::decode_jwt;
use futures_util::{StreamExt, SinkExt};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct WsQuery {
    token: String,
    room_id: String,
}

type Tx = broadcast::Sender<(String, String)>;

lazy_static::lazy_static! {
    static ref ROOMS: Arc<Mutex<HashMap<Uuid, Tx>>> = Arc::new(Mutex::new(HashMap::new()));
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let claims = match decode_jwt(&query.token) {
        Ok(c) => c,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let user_id = claims.sub;
    let room_id = match Uuid::parse_str(&query.room_id) {
        Ok(id) => id,
        Err(_) => return axum::http::StatusCode::BAD_REQUEST.into_response(),
    };

    let tx = {
        let mut rooms = ROOMS.lock().unwrap();
        rooms
            .entry(room_id)
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tx, room_id, user_id, pool))
}

async fn handle_socket(socket: WebSocket, tx: Tx, room_id: Uuid, user_id: String, pool: PgPool) {
    let mut rx = tx.subscribe();
    let (mut sender, mut receiver) = socket.split();
    let user_id_clone = user_id.clone();

    let send_task = tokio::spawn(async move {
        while let Ok((uid, msg)) = rx.recv().await {
            if uid != user_id_clone {
                let _ = sender
                    .send(Message::Text(
                        serde_json::to_string(&serde_json::json!({
                            "type": "message",
                            "payload": {
                                "sender_id": uid,
                                "content": msg,
                            }
                        }))
                        .unwrap()
                        .into(),
                    ))
                    .await;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(txt))) = receiver.next().await {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let Some(content) = val.get("content").and_then(|c| c.as_str()) {
                    let _ = tx.send((user_id.clone(), content.to_string()));

                    if let Ok(sender_uuid) = Uuid::parse_str(&user_id) {
                        let _ = sqlx::query!(
                            "INSERT INTO messages (sender_id, room_id, content) VALUES ($1, $2, $3)",
                            sender_uuid,
                            room_id,
                            content
                        )
                        .execute(&pool)
                        .await;
                    }
                }
            }
        }
    });

    let _ = tokio::try_join!(send_task, recv_task);
}
