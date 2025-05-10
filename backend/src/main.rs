use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any };
use http::Method;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tower::make::Shared;

mod auth;
mod routes;
mod db;
mod route_handlers;
mod models;
mod state;

use crate::state::AppState;
use crate::routes::{create_routes,ws_routes};


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();


    let db_pool = db::init_db()
        .await
        .expect("Failed to connect to the database");

    let app_state = Arc::new(AppState {
        pool: db_pool.clone(),
            rooms: Arc::new(RwLock::new(HashMap::new())),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_headers(Any);
    

    let app= Router::new()
        .route("/api/health", get(|| async { "Alive!" }))
        .merge(create_routes(app_state.clone()))
        .merge(ws_routes(app_state.clone()))
        .layer(cors)
        .with_state(app_state.clone());

    let service = Shared::new(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Failed to bind to port 4000");

    println!("🚀 Server running at http://0.0.0.0:4000");
    axum::serve(listener, service)
        .await
        .expect("server crashed");
}
