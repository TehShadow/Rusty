use axum::{extract::State, Json};
use axum::http::{StatusCode,HeaderMap};
use axum::response::IntoResponse;
use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration;
use crate::models::user::{LoginInput,RegisterInput,User};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use uuid::Uuid;
use time::OffsetDateTime;
use crate::auth::jwt::create_jwt;
use crate::state::AppState;
use std::sync::Arc;


pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterInput>,
) -> Result<Json<User>, (StatusCode, String)> {
    // Generate password hash
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt.into())
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hashing error".into()))?
        .to_string();

    // Insert user
    let user_id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();

    sqlx::query!(
        r#"
        INSERT INTO users (id, username, password_hash, created_at)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        payload.username,
        password_hash,
        now
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(User {
        id: user_id,
        username: payload.username,
        password_hash,
        created_at: now,
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Step 1: Fetch user manually
    let row = sqlx::query!(
        r#"
        SELECT id, username, password_hash
        FROM users
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row = match row {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid username or password".into())),
    };

    // Step 2: Verify password
    let parsed_hash = PasswordHash::new(&row.password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Corrupt password hash")).unwrap();

    let verified = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !verified {
        return Err((StatusCode::UNAUTHORIZED, "Invalid username or password".into()));
    }
    

    // Step 3: Create opaque session token
    let session_token: String = Uuid::new_v4().to_string();
    let jwt = create_jwt(&row.id.to_string(),&session_token,&payload.username);
    let now = OffsetDateTime::now_utc();
    let expires_at = now + Duration::days(2);
    let max_age = Duration::days(7);

    sqlx::query!(
        r#"
        INSERT INTO sessions (token, user_id, created_at, expires_at)
        VALUES ($1, $2, $3, $4)
        "#,
        session_token,
        row.id,
        now,
        expires_at
    )
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Step 5: Create session cookie
    let cookie = Cookie::build(("token", &jwt ))
        .http_only(true)
        .secure(true) // set false if testing on localhost
        .same_site(SameSite::Strict)
        .path("/")
        .max_age(max_age);


    // Step 6: Respond with JWT + set-cookie
    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", cookie.to_string().parse().unwrap());

    let body = Json(serde_json::json!({ "token": jwt }));

    Ok((headers, body))
}