use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use serde::{Serialize, Deserialize};
use std::env;
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


// Used in protected route to decode the JWT
pub fn decode_jwt(token: &str) -> Result<Claims, JwtError> {
    let key = DecodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &key, &validation)?;
    Ok(token_data.claims)
}
pub fn create_jwt(user_id: String) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp();

    let claims = Claims {
        sub: user_id,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env::var("JWT_SECRET").unwrap().as_bytes()),
    )
    .unwrap()
}
