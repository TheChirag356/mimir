use crate::{auth::jwt::decode_token, db, error::AppError, state::AppState};
use axum::{
    body::Body,
    extract::{Path, Query, Request, State},
    response::IntoResponse,
};
use std::collections::HashMap;
use tower::ServiceExt;
use tower_http::services::ServeFile;

pub async fn serve_ebook(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    request: Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    // Same dual-auth pattern as audio stream
    if let Some(t) = params.get("token") {
        decode_token(t, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    } else {
        let auth_header = request
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("missing token".into()))?;
        decode_token(auth_header, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    }

    let ebook = db::ebook_files::get_ebook_for_item(&state.db, &item_id)
        .await?
        .ok_or_else(|| AppError::NotFound("no ebook found for this item".into()))?;

    ServeFile::new(&ebook.file_path)
        .oneshot(request)
        .await
        .map_err(|_| AppError::NotFound("ebook file could not be read".into()))
}
