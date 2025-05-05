use axum::{Router, routing::{post, get}, Json, http::StatusCode};
use crate::auth::handlers::{register, login};
use crate::auth::middleware::{auth_middleware, USER};
use crate::route_handlers::me::me_handler;


pub fn create_routes() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me_handler).route_layer(axum::middleware::from_fn(auth_middleware)))
}
