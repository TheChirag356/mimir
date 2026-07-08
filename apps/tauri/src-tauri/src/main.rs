#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn login(
    server_url: String,
    username: String,
    password: String,
) -> Result<serde_json::Value, String> {
    eprintln!("login attempt: server={server_url} username={username}");
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{server_url}/api/login"))
        .json(&serde_json::json!({ "username": username, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_libraries(server_url: String, token: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{server_url}/api/libraries"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_library_items(
    server_url: String,
    token: String,
    library_id: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{server_url}/api/libraries/{library_id}/items"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_all_items(server_url: String, token: String) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let libs_response = client
        .get(format!("{server_url}/api/libraries"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let libraries: Vec<serde_json::Value> =
        libs_response.json().await.map_err(|e| e.to_string())?;

    let mut all_items: Vec<serde_json::Value> = vec![];

    for lib in libraries {
        if let Some(lib_id) = lib["id"].as_str() {
            let items_response = client
                .get(format!("{server_url}/api/libraries/{lib_id}/items"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .map_err(|e| e.to_string())?;

            if let Ok(items) = items_response.json::<serde_json::Value>().await {
                if let Some(arr) = items.as_array() {
                    all_items.extend(arr.clone());
                }
            }
        }
    }

    Ok(serde_json::Value::Array(all_items))
}

#[tauri::command]
async fn get_item(
    server_url: String,
    token: String,
    item_id: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{server_url}/api/items/{item_id}"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn start_session(
    server_url: String,
    token: String,
    item_id: String,
    duration_seconds: f64,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{server_url}/api/items/{item_id}/play"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({ "duration_seconds": duration_seconds }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn sync_session(
    server_url: String,
    token: String,
    session_id: String,
    current_time_seconds: f64,
    time_listened_seconds: f64,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .patch(format!("{server_url}/api/session/{session_id}/sync"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "current_time_seconds": current_time_seconds,
            "time_listened_seconds": time_listened_seconds
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_item_progress(
    server_url: String,
    token: String,
    item_id: String,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{server_url}/api/items/{item_id}/progress"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_ebook_url(
    server_url: String,
    token: String,
    item_id: String,
) -> Result<String, String> {
    Ok(format!(
        "{server_url}/api/items/{item_id}/ebook?token={token}"
    ))
}

#[tauri::command]
async fn sync_ebook_progress(
    server_url: String,
    token: String,
    item_id: String,
    cfi: String,
    progress_percent: f64,
) -> Result<serde_json::Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .patch(format!("{server_url}/api/items/{item_id}/ebook/progress"))
        .header("Authorization", format!("Bearer {token}"))
        .json(&serde_json::json!({
            "progress_percent": progress_percent,
            "cfi": cfi
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login,
            get_libraries,
            get_library_items,
            get_all_items,
            get_item,
            start_session,
            sync_session,
            get_item_progress,
            get_ebook_url,
            sync_ebook_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
