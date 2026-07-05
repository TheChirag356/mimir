mod api;
mod auth;
mod db;
mod error;
mod scanner;
mod state;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Build a connection pool rather than a single connection — this lets
    // multiple concurrent requests each grab a connection without
    // blocking each other.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    let state = AppState {
        db: pool,
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route(
            "/api/setup",
            axum::routing::post(api::auth::create_first_user),
        )
        .route("/api/login", post(api::auth::login))
        .route(
            "/api/libraries",
            get(api::libraries::list_libraries).post(api::libraries::create_library),
        )
        .route(
            "/api/libraries/{id}",
            get(api::libraries::get_library)
                .patch(api::libraries::update_library)
                .delete(api::libraries::delete_library),
        )
        .route(
            "/api/libraries/{id}/scan",
            post(api::libraries::scan_library),
        )
        .route(
            "/api/libraries/{id}/folders",
            get(api::libraries::list_folders).post(api::libraries::create_folder),
        )
        .route(
            "/api/items/{item_id}/audio/{audio_file_id}/stream",
            get(api::stream::stream_audio),
        )
        .route("/api/items/{id}/play", post(api::session::start_session))
        .route("/api/session/{id}/sync", patch(api::session::sync_session))
        .route(
            "/api/items/{id}/progress",
            get(api::session::get_item_progress),
        )
        .route("/api/items/{id}/ebook", get(api::ebook::serve_ebook))
        .route(
            "/api/items/{id}/ebook/progress",
            patch(api::session::sync_ebook_progress),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();

    println!("mimir-core listening on http://0.0.0.0:3333");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "ok"
}
