use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use sqlx::PgPool;
use uuid::Uuid;

pub type Tx = broadcast::Sender<String>;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub rooms: Arc<RwLock<HashMap<Uuid, Tx>>>,
}