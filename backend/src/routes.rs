use axum::{
    Router,
    routing::{post, get, delete},
    middleware::from_fn,
};
use crate::auth::handlers::{login, register};
use crate::auth::middleware::auth_middleware;

use crate::route_handlers::me::me_handler;
use crate::route_handlers::ws::ws_handler;
use crate::route_handlers::users::get_users;
use crate::route_handlers::rooms::{
    create_room, 
    list_rooms, 
    list_room_members, 
    add_room_member
};
use crate::route_handlers::messages::{post_message_to_room, get_messages_by_room};
use crate::route_handlers::req_friends::{
    add_friend_request,
    get_friend_requests,
    accept_friend_request,
    delete_friend_request,
    list_friends,
};

use sqlx::PgPool;

pub fn create_routes() -> Router<PgPool> {
    // 🛡️ Protected routes (require auth middleware)
    let protected_routes = Router::new()
        // Profile / identity
        .route("/me", get(me_handler))

        // Users
        .route("/users", get(get_users))

        // Rooms
        .route("/users/{user_id}/rooms", post(create_room).get(list_rooms))
        .route("/rooms/{room_id}/members", get(list_room_members).post(add_room_member))
        .route("/rooms/{room_id}/messages", get(get_messages_by_room).post(post_message_to_room))

        // Friendships
        .route("/friends/{user_id}", get(list_friends).post(add_friend_request))
        .route("/friends/{user_id}/requests", get(get_friend_requests))
        .route("/friends/{user_i}/requests/{request_id}/accept", post(accept_friend_request))
        .route("/friends/{user_i}/requests/{request_id}", delete(delete_friend_request));

    // 🆓 Public routes (no auth)
    let unprotected_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))

        // WebSocket: open access (auth handled in handler logic)
        .route("/ws/{room_id}", get(ws_handler));

    // 🔀 Compose everything together with auth layer on protected routes
    Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes.route_layer(from_fn(auth_middleware)))
}