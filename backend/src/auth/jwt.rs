use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use serde::{Serialize, Deserialize};
use std::env;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,            // user ID
    pub username: String,       // optional identity
    pub session_id: String,     // UUID of DB session
    pub exp: usize,             // expiration timestamp
}

pub fn create_jwt(user_id: &str, session_id: &str, username: &str) -> String {
    let exp = Utc::now().timestamp() + 60 * 60; // 1 hour
    let claims = Claims {
        sub: user_id.to_string(),
        session_id: session_id.to_string(),
        exp: exp as usize,
        username: username.to_string()
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).expect("JWT creation failed")
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(), // requires exp by default
    );

    if let Err(ref err) = result {
        eprintln!("❌ JWT decode error: {:?}", err);
    }

    result
}