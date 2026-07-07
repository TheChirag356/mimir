// Prevents console window from appearing on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Tauri "commands" are async Rust functions callable directly from
// Svelte via `invoke("command_name", { args })`. They're the bridge
// between your frontend and mimir-client's Rust logic — no REST
// calls needed between your own UI and its own backend.
#[tauri::command]
async fn get_libraries(
    server_url: String,
    token: String,
) -> Result<Vec<serde_json::Value>, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{server_url}/api/libraries"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // mimir-core returns an array directly for /api/libraries
    let libraries: Vec<serde_json::Value> = response.json().await.map_err(|e| e.to_string())?;

    Ok(libraries)
}

#[tauri::command]
async fn login(
    server_url: String,
    username: String,
    password: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{server_url}/api/login"))
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![login, get_libraries])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
