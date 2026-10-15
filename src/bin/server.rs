use axum::{extract::State, http::StatusCode, routing::get, Json, Router,};
use anyhow::{Context, Result};
use std::{env, net::SocketAddr};
use serde::Serialize;
use sqlx::{ PgPool, FromRow};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let http_addr: SocketAddr = env::var("HTTP_ADDR")
        .context("HTTP_ADDR is missing")?
        .parse()
        .context("HTTP_ADDR must be an IP address and port")?;

    let database_url = env::var("DATABASE_URL")
        .context("DATABASE_URL is missing")?;

    let listener = tokio::net::TcpListener::bind(http_addr).await?;

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to the PostgreSQL")?;

    println!("Connected to PostgreSQL successfully");

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/categories", get(get_categories))
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

#[derive(Serialize, FromRow)]
struct Category {
    id: i32,
    name: String,
    parent_id: Option<i32>,
}

async fn get_categories(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, parent_id FROM categories ORDER BY id",
    )
        .fetch_all(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to fetch categories: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(categories))
}