use crate::{db, error::AppError, state::AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mimir_common::library::{CreateLibraryRequest, LibraryDto, UpdateLibraryRequest};
use sqlx::SqlitePool;

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
