use crate::{
    auth::{extractor::AuthUser, jwt::decode_token},
    db,
    error::AppError,
    state::AppState,
};
use axum::{
    body::Body,
    extract::{Path, Query, Request, State},
    response::IntoResponse,
};
use std::collections::HashMap;
use tower::ServiceExt;
use tower_http::services::ServeFile;

pub async fn stream_audio(
    State(state): State<AppState>,
    Path((item_id, audio_file_id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    request: Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    // Validate auth — either Bearer header (via AuthUser) or ?token= query param
    // (needed for HTMLAudioElement which can't set custom headers)
    let token = params.get("token").cloned();
    if let Some(t) = token {
        decode_token(&t, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    } else {
        // If no query token, require the Authorization header
        let auth_header = request
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized("missing token".into()))?;
        decode_token(auth_header, &state.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid token".into()))?;
    }

    let audio_file = db::audio_files::get_audio_file_for_item(&state.db, &item_id, &audio_file_id)
        .await?
        .ok_or_else(|| AppError::NotFound("audio file not found".into()))?;

    ServeFile::new(&audio_file.file_path)
        .oneshot(request)
        .await
        .map_err(|_| AppError::NotFound("file could not be read".into()))
}
