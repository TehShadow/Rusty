use axum::{
    routing::{get, post},
    Router,
};


mod auth;
mod routes;
mod db;
mod route_handlers;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_pool = db::init_db()
        .await
        .expect("Failed to connect to the database");

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(routes::create_routes())
        .with_state(db_pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("🚀 Server running at http://0.0.0.0:3000");
    axum::serve(listener, app)
        .await
        .expect("Server crashed");
}
