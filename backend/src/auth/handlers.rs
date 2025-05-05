use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;
use argon2::{Argon2, PasswordHasher,PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng, PasswordHash};
use crate::auth::models::{RegisterRequest,LoginRequest,TokenResponse};
use crate::auth::jwt::create_jwt;


pub async fn register(
    State(db): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> StatusCode {
    // Hash the password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(payload.password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string();

    // Insert into DB
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        "#,
        payload.username,
        password_hash
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            eprintln!("Registration error: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn login(
    State(db): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {
    // Step 1: Look up user by username
    let user = sqlx::query!(
        r#"
        SELECT id, password_hash
        FROM users
        WHERE username = $1
        "#,
        payload.username
    )
    .fetch_optional(&db)
    .await
    .map_err(|e| {
        eprintln!("DB error during login: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = match user {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Step 2: Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let password_ok = Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !password_ok {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Step 3: Issue JWT
    let token = create_jwt(user.id.to_string());

    Ok(Json(TokenResponse { token }))
}

