use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}
