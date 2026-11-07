use anyhow::{Context, Result};
use axum::routing::{post, put};
use axum::{Json, Router, routing::get};
use serde::Serialize;
use std::{env, net::SocketAddr};

mod catalog;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let http_addr: SocketAddr = env::var("HTTP_ADDR")
        .context("HTTP_ADDR is missing")?
        .parse()
        .context("HTTP_ADDR must be an IP address and port")?;

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is missing")?;

    let listener = tokio::net::TcpListener::bind(http_addr).await?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to the PostgreSQL")?;

    println!("Connected to PostgreSQL successfully");

    let app = Router::new()
        .route("/health", get(health))
        .nest("/api/v1/categories", catalog::categories::category_routes())
        .nest("/api/v1/weapons", catalog::weapons::weapon_routes())
        .with_state(pool);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "OK" })
}
