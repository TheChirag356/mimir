use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// What actually gets encoded inside the token. Keep this minimal —
// anything in here is readable by anyone who has the token (JWTs are
// signed, not encrypted), so no passwords or sensitive data belong here
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // "subject" — the user ID, a JWT convention
    pub username: String,
    pub is_admin: bool,
    pub exp: i64, // expiry, unix timestamp — required by jsonwebtoken
}

pub fn create_token(
    user_id: &str,
    username: &str,
    is_admin: bool,
    secret: &str,
) -> Result<String, String> {
    let expiry = chrono::Utc::now() + chrono::Duration::days(30);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        is_admin,
        exp: expiry.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| e.to_string())
}

pub fn decode_token(token: &str, secret: &str) -> Result<Claims, String> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}
