use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{CorsLayer, Any };
use http::Method;

mod auth;
mod routes;
mod db;
mod route_handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();


    let db_pool = db::init_db()
        .await
        .expect("Failed to connect to the database");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::GET])
        .allow_headers(Any);
    

    let app = Router::new()
        .route("/api/health", get(|| async { "Alive!" }))
        .merge(routes::create_routes(db_pool.clone()))
        .layer(cors)
        .with_state(db_pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Failed to bind to port 4000");

    println!("🚀 Server running at http://0.0.0.0:4000");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server crashed");
}
