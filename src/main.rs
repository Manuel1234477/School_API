mod controllers;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use routes::auth_routes;
use utils::JwtConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/school_db".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    tracing::info!("Database migrations completed successfully");

    // JWT configuration
    let jwt_config = JwtConfig::from_env();

    // Build router
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        
        // Authentication routes
        .nest("/auth", auth_routes(pool.clone(), jwt_config.clone()))
        
        // CORS layer
        .layer(CorsLayer::permissive());

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    let response = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    (StatusCode::OK, Json(response))
}
