use serde::{Serialize, Deserialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct PublicUser{
    pub id: Uuid,
    pub username: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct RegisterInput {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[derive(Serialize , Deserialize)]
pub struct SimpleUser {
    pub id: Uuid,
    pub username: String,
}