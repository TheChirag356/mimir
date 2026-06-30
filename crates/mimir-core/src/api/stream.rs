use crate::{auth::extractor::AuthUser, db, error::AppError, state::AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    http::Request,
    response::IntoResponse,
};
use tower::ServiceExt; // brings `.oneshot()` into scope
use tower_http::services::ServeFile;

pub async fn stream_audio(
    State(state): State<AppState>,
    Path((item_id, audio_file_id)): Path<(String, String)>,
    _user: AuthUser,
    request: Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    let audio_file = db::audio_files::get_audio_file_for_item(&state.db, &item_id, &audio_file_id)
        .await?
        .ok_or_else(|| AppError::NotFound("audio file not found".into()))?;

    // `ServeFile` is a `tower::Service` that knows how to read a file
    // off disk and respond correctly to `Range` headers — returning
    // `206 Partial Content` with the right `Content-Range` when asked,
    // or the full file with `200 OK` otherwise. We don't have to
    // implement any of that byte-range logic ourselves.
    let serve_file = ServeFile::new(&audio_file.file_path);

    // `.oneshot(request)` runs this Service exactly once against the
    // incoming request (which still carries the original `Range`
    // header, since we took `Request<Body>` directly instead of using
    // an extractor that would have consumed/discarded those headers).
    let response = serve_file
        .oneshot(request)
        .await
        .map_err(|_| AppError::NotFound("file could not be read".into()))?;

    Ok(response)
}
