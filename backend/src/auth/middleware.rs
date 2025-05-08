use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::auth::jwt::decode_jwt;
use crate::auth::current_user::CurrentUser;

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header.strip_prefix("Bearer ").ok_or(StatusCode::UNAUTHORIZED)?;
    let claims = decode_jwt(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = CurrentUser {
        user_id: claims.sub,
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}