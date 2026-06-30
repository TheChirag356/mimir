use crate::auth::extractor::AuthUser;
use crate::scanner;
use crate::{db, error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mimir_common::library::{CreateFolderRequest, LibraryFolderDto};
use mimir_common::library::{CreateLibraryRequest, LibraryDto, UpdateLibraryRequest};

impl From<db::models::library::Library> for LibraryDto {
    fn from(lib: db::models::library::Library) -> Self {
        LibraryDto {
            id: lib.id,
            name: lib.name,
            media_type: lib.media_type,
            created_at: lib.created_at,
            updated_at: lib.updated_at,
        }
    }
}

impl From<db::models::library_folder::LibraryFolder> for LibraryFolderDto {
    fn from(f: db::models::library_folder::LibraryFolder) -> Self {
        LibraryFolderDto {
            id: f.id,
            library_id: f.library_id,
            path: f.path,
        }
    }
}

fn validate_media_type(media_type: &str) -> Result<(), AppError> {
    if media_type != "book" && media_type != "podcast" {
        return Err(AppError::BadRequest(
            "media_type must be a 'book' or 'podcast'".into(),
        ));
    }
    Ok(())
}

pub async fn list_libraries(
    State(state): State<AppState>,
    _user: AuthUser, // present but unused — just requires a valid token
) -> Result<Json<Vec<LibraryDto>>, AppError> {
    let libraries = db::libraries::list_libraries(&state.db).await?;
    Ok(Json(libraries.into_iter().map(Into::into).collect()))
}

pub async fn get_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<LibraryDto>, AppError> {
    let library = db::libraries::get_library(&state.db, &id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("get_library: library {id} not found")))?;
    Ok(Json(library.into()))
}

pub async fn create_library(
    State(state): State<AppState>,
    Json(payload): Json<CreateLibraryRequest>,
) -> Result<Json<LibraryDto>, AppError> {
    validate_media_type(&payload.media_type)?;
    let library =
        db::libraries::create_library(&state.db, &payload.name, &payload.media_type).await?;
    Ok(Json(library.into()))
}

pub async fn update_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateLibraryRequest>,
) -> Result<Json<LibraryDto>, AppError> {
    if let Some(ref mt) = payload.media_type {
        validate_media_type(mt)?;
    }

    let updated = db::libraries::update_library(
        &state.db,
        &id,
        payload.name.as_deref(),
        payload.media_type.as_deref(),
    )
    .await?
    .ok_or_else(|| AppError::NotFound(format!("library {id} not found")))?;

    Ok(Json(updated.into()))
}

pub async fn delete_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let deleted = db::libraries::delete_library(&state.db, &id).await?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound(format!("library {id} not found")))
    }
}

pub async fn scan_library(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let summary = scanner::scan_library(&state.db, &id).await?;
    Ok(Json(serde_json::json!({
        "items_added": summary.items_added,
        "items_skipped": summary.items_skipped
    })))
}

pub async fn create_folder(
    State(state): State<AppState>,
    Path(library_id): Path<String>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<LibraryFolderDto>, AppError> {
    // Confirm the library itself exists first — otherwise you'd silently
    // create an orphaned folder row pointing at a nonexistent library.
    db::libraries::get_library(&state.db, &library_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("library {library_id} not found")))?;

    // `tokio::fs::canonicalize` does two things at once: it resolves the
    // path to an absolute form (so "./audiobooks" and "/home/you/audiobooks"
    // don't end up as two different DB rows pointing at the same place),
    // and it fails with an error if the path doesn't actually exist —
    // which is exactly the validation we want here.
    let canonical = tokio::fs::canonicalize(&payload.path)
        .await
        .map_err(|_| AppError::BadRequest(format!("path does not exist: {}", payload.path)))?;

    let metadata = tokio::fs::metadata(&canonical)
        .await
        .map_err(|_| AppError::BadRequest("could not read path metadata".into()))?;

    if !metadata.is_dir() {
        return Err(AppError::BadRequest(format!(
            "not a directory: {}",
            payload.path
        )));
    }

    let folder =
        db::library_folders::create_folder(&state.db, &library_id, &canonical.to_string_lossy())
            .await?;

    Ok(Json(folder.into()))
}

pub async fn list_folders(
    State(state): State<AppState>,
    Path(library_id): Path<String>,
) -> Result<Json<Vec<LibraryFolderDto>>, AppError> {
    let folders = db::library_folders::list_folders(&state.db, &library_id).await?;
    Ok(Json(folders.into_iter().map(Into::into).collect()))
}
