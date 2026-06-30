mod api;
mod auth;
mod db;
mod error;
mod scanner;
mod state;

use axum::{routing::get, Router};
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
        .route("/api/login", axum::routing::post(api::auth::login))
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
            axum::routing::post(api::libraries::scan_library),
        )
        .route(
            "/api/libraries/{id}/folders",
            get(api::libraries::list_folders).post(api::libraries::create_folder),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await.unwrap();

    println!("mimir-core listening on http://0.0.0.0:3333");
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "ok"
}
