use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    #[serde(rename = "isInit")]
    pub is_init: bool,
    pub language: String,
}

#[derive(Serialize)]
pub struct PingResponse {
    pub success: bool,
}

pub async fn status() -> Json<StatusResponse> {
    Json(StatusResponse {
        is_init: true,
        language: "en-us".to_string(),
    })
}

pub async fn ping() -> Json<PingResponse> {
    Json(PingResponse { success: true })
}

pub async fn healthcheck() -> &'static str {
    "OK"
}
