use super::jwt::decode_token;
use crate::{error::AppError, state::AppState};
use axum::{extract::FromRequestParts, http::request::Parts};

// Any handler that takes `AuthUser` as a parameter automatically
// requires a valid token — Axum runs this extraction *before* the
// handler body, so an invalid/missing token never even reaches your
// route logic.
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    // Axum's extractor traits use native `async fn in trait` as of
    // recent versions — no `#[async_trait]` macro needed here, unlike
    // in a lot of older Rust web tutorials you might come across.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("missing Authorization header".into()))?;

        let token = header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("expected Bearer token".into()))?;

        let claims = decode_token(token, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid or expired token".into()))?;

        Ok(AuthUser {
            id: claims.sub,
            username: claims.username,
            is_admin: claims.is_admin,
        })
    }
}
