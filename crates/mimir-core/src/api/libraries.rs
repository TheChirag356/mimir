use crate::{db, state::AppState};
use axum::{extract::State, Json};

pub async fn get_libraries(State(state): State<AppState>) -> Json<serde_json::Value> {
    match db::libraries::list_libraries(&state.db).await {
        Ok(libraries) => Json(serde_json::json!({ "libraries": libraries })),
        Err(e) => {
            eprintln!("DB error: {e}");
            Json(serde_json::json!({"libraries": []}))
        }
    }
}
