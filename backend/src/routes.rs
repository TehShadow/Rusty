use axum::{Router, routing::{post, get}, Json, http::StatusCode};
use crate::{auth::handlers::{login, register}, route_handlers};
use crate::auth::middleware::{auth_middleware, USER};
use crate::route_handlers::me::me_handler;
use crate::route_handlers::ws::ws_handler;
use crate::route_handlers::users::get_users;
use crate::route_handlers::rooms::{
    create_room, list_rooms, get_messages_by_room, send_message_to_room,
};

pub fn create_routes() -> Router<sqlx::PgPool> {
    let protected_routes = Router::new()
        .route("/me", get(me_handler))
        .route("/users", get(get_users))
        .route("/rooms", post(create_room).get(list_rooms))
        .route("/rooms/{room_id}/messages", get(get_messages_by_room).post(send_message_to_room))
        .route("/ws", get(ws_handler));

    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .merge(protected_routes.route_layer(axum::middleware::from_fn(auth_middleware)))
}
