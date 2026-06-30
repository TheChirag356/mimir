use crate::{
    auth::{
        jwt::create_token,
        password::{hash_password, verify_password},
    },
    db,
    error::AppError,
    state::AppState,
};
use axum::{extract::State, Json};
use mimir_common::auth::{CreateUserRequest, LoginRequest, LoginResponse, UserDto};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = db::users::get_user_by_username(&state.db, &payload.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid username or password".into()))?;

    if !verify_password(&payload.password, &user.password_hash) {
        // Deliberately the same error as "user not found" above — never
        // reveal whether a username exists via a different error
        // message, that's a basic username-enumeration leak.
        return Err(AppError::Unauthorized(
            "invalid username or password".into(),
        ));
    }

    let token = create_token(&user.id, &user.username, user.is_admin, &state.jwt_secret)
        .map_err(|_| AppError::Unauthorized("failed to issue token".into()))?;

    Ok(Json(LoginResponse {
        token,
        user: UserDto {
            id: user.id,
            username: user.username,
            is_admin: user.is_admin,
        },
    }))
}

// Bootstrap-only: allows creating the *first* user with no auth required.
// Once any user exists, this route refuses to create another — further
// user creation should go through an admin-protected route you'll add
// when building out user management properly.
pub async fn create_first_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserDto>, AppError> {
    let existing_count = db::users::count_users(&state.db).await?;
    if existing_count > 0 {
        return Err(AppError::BadRequest("setup already completed".into()));
    }

    let hash = hash_password(&payload.password)
        .map_err(|_| AppError::BadRequest("failed to hash password".into()))?;

    let user = db::users::create_user(&state.db, &payload.username, &hash, true).await?;

    Ok(Json(UserDto {
        id: user.id,
        username: user.username,
        is_admin: user.is_admin,
    }))
}
