use axum::{Router, routing::{post, get}};
use crate::auth::handlers::{register, login};
use crate::auth::middleware::{auth_middleware, USER};

async fn protected() -> String {
    let user = USER.with(|u| u.clone());
    format!("Hello, user {}!", user.user_id)
}

pub fn create_routes() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(protected).route_layer(axum::middleware::from_fn(auth_middleware)))
}
