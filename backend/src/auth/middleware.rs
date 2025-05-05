use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::auth::jwt::decode_jwt;

#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub user_id: String,
}

tokio::task_local! {
    pub static USER: CurrentUser;
}

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
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

    Ok(USER.scope(user, next.run(req)).await)
}
