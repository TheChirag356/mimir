use crate::{auth::extractor::AuthUser, db, error::AppError, state::AppState};
use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::IntoResponse,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

pub async fn serve_ebook(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
    _user: AuthUser,
    request: Request<Body>,
) -> Result<impl IntoResponse, AppError> {
    let ebook = db::ebook_files::get_ebook_for_item(&state.db, &item_id)
        .await?
        .ok_or_else(|| AppError::NotFound("no ebook found for this item".into()))?;

    ServeFile::new(&ebook.file_path)
        .oneshot(request)
        .await
        .map_err(|_| AppError::NotFound("ebook file could not be read".into()))
}
