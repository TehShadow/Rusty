use axum::{
    middleware:: from_fn_with_state,
    routing::{get, post},
    Router,
};
use crate::auth::handlers::{login, register};
use crate::route_handlers::me::get_me;
use crate::route_handlers::users::get_user_by_id;
use crate::route_handlers::users::{get_direct_messages,send_direct_message};
use crate::route_handlers::room::{create_room,join_room,list_my_rooms,send_room_message,get_room_messages};

use crate::auth::middleware::auth_middleware;
use sqlx::PgPool;

pub fn create_routes(pool: PgPool) -> Router<PgPool> {
    // 🛡️ Protected routes (require auth middleware)
    let protected_routes = Router::new()
        // Profile / identity / DMs
        .route("/api/me", get(get_me))
        .route("/api/user/{:user}",get(get_user_by_id))
        .route("/api/user/dm/{:user}",get(get_direct_messages).post(send_direct_message))

        // Rooms

        .route("/api/rooms", post(create_room).get(list_my_rooms))
        .route("/api/rooms/{:id}/join", post(join_room))
        .route("/api/rooms/{:id}/messages", get(get_room_messages).post(send_room_message));

    // 🆓 Public routes (no auth)
    let unprotected_routes = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login));

    // 🔀 Compose everything together
    Router::new()
        .merge(unprotected_routes)
        .merge(protected_routes.route_layer(from_fn_with_state(pool,auth_middleware)))
}
