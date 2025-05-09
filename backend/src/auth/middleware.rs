use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::auth::jwt::decode_jwt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub username: String,
    pub session_id: Uuid,
}

pub async fn auth_middleware(
    State(pool): State<PgPool>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, &'static str)> {
    // Extract token from headers
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header"))?;

    println!("here {token}");

    // Decode JWT
    let claims = decode_jwt(token)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired JWT"))?
        .claims;
    

    // Parse UUIDs
    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid user ID"))?;
    let session_id = Uuid::parse_str(&claims.session_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid session ID"))?;

    println!("{user_id}");

    println!("{session_id}");

    // Verify session is valid
    let valid_session = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM sessions
            WHERE token = $1 AND user_id = $2 AND expires_at > NOW()
        ) AS "exists!"
        "#,
        session_id.to_string(),
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error"))?;

    if !valid_session {
        return Err((StatusCode::UNAUTHORIZED, "Session is invalid or expired"));
    }

    // Add user to request extensions
    request.extensions_mut().insert(CurrentUser {
        id: user_id,
        username: claims.username,
        session_id,
    });

    // Continue with the request
    Ok(next.run(request).await)
}

