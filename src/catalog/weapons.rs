use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Weapon {
    id: i32,
    name: String,
    category_id: i32
}

pub async fn get_weapons(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Weapon>>, StatusCode> {
    let weapons = sqlx::query_as::<_, Weapon>(
        "SELECT id, name, category_id FROM weapons ORDER BY id",
    )
        .fetch_all(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to fetch categories: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(weapons))
}