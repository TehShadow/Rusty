use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::IntoResponse,
};
use futures_util::{StreamExt, SinkExt};

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("WebSocket connected");

    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                println!(">> Received: {}", text);
                if let Err(e) = socket.send(Message::Text(format!("Echo: {text}").into())).await {
                    eprintln!("WebSocket send error: {}", e);
                    return;
                }
            }
            Message::Close(_) => {
                println!("WebSocket closed by client");
                return;
            }
            _ => {}
        }
    }

    println!("WebSocket connection dropped");
}
