use axum::{
    middleware::from_fn_with_state,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::auth::handlers::{login, register};
use crate::route_handlers::me::get_me;
use crate::route_handlers::users::{get_user_by_id, get_direct_messages, send_direct_message};
use crate::route_handlers::room::{
    create_room, join_room, list_my_rooms, send_room_message, get_room_messages, get_room, list_room_members
};
use crate::route_handlers::relationships::{
    send_friend_request, accept_friend_request, block_user, list_friends,
    remove_relationship, list_pending_requests,
};
use crate::route_handlers::ws::{ws_handler,ws_dm_handler};
use crate::auth::middleware::auth_middleware;
use crate::state::AppState;

pub fn create_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    let protected_routes = Router::new()
        //users
        .route("/api/me", get(get_me))
        .route("/api/user/{:user}", get(get_user_by_id))
        .route("/api/dm/{:user}", get(get_direct_messages).post(send_direct_message))
        //relationships
        .route("/api/relationships/{:id}", post(send_friend_request).delete(remove_relationship))
        .route("/api/relationships/{:id}/accept", post(accept_friend_request))
        .route("/api/relationships/{:id}/block", post(block_user))
        .route("/api/relationships/friends", get(list_friends))
        .route("/api/relationships/pending", get(list_pending_requests))
        //Rooms
        .route("/api/rooms/{:id}", get(get_room))
        .route("/api/rooms", post(create_room).get(list_my_rooms))
        .route("/api/rooms/{:id}/join", post(join_room))
        .route("/api/rooms/{:id}/messages", get(get_room_messages).post(send_room_message))
        .route("/api/rooms/{:id}/members",get(list_room_members));

    let unprotected_routes = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login));

    Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes.route_layer(from_fn_with_state(app_state.clone(), auth_middleware)))
}

pub fn ws_routes(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/ws/{:room_id}", get(ws_handler))
        .route("/api/dm/ws/{other_user_id}", get(ws_dm_handler))
        .with_state(app_state)
}
