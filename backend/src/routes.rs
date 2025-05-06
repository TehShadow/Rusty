use axum::{Router, routing::{post, get}};
use crate::auth::handlers::{login, register};
use crate::auth::middleware::auth_middleware;
use crate::route_handlers::me::me_handler;
use crate::route_handlers::ws::ws_handler;
use crate::route_handlers::users::get_users;
use crate::route_handlers::rooms::{
    create_room, list_rooms , list_room_members , add_room_member
};
use crate::route_handlers::friendships::{
    add_friend , list_friends
};

use crate::route_handlers::messages::{
    post_message_to_room , get_messages_by_room
};

pub fn create_routes() -> Router<sqlx::PgPool> {
    let protected_routes = Router::new()
        .route("/friends",get(list_friends).post(add_friend))
        .route("/me", get(me_handler))
        .route("/users", get(get_users))
        .route("/rooms", post(create_room).get(list_rooms))
        .route("/rooms/{roomd_id}/members",get(list_room_members).post(add_room_member))
        .route("/rooms/{room_id}/messages", get(get_messages_by_room).post(post_message_to_room));
        

    let unprotected_routes = Router::new()
    .route("/register", post(register))
    .route("/login", post(login))
    .route("/ws/{room_id}", get(ws_handler));

    Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes.route_layer(axum::middleware::from_fn(auth_middleware)))
}
