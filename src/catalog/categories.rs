use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

#[derive(Serialize, FromRow)]
pub struct Category {
    id: i32,
    name: String,
    parent_id: Option<i32>,
}

pub async fn get_categories(
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

#[derive(Deserialize)]
pub struct create_category_request {
    name: String,
    parent_id: Option<i32>
}

pub async fn create_category(
    State(pool): State<PgPool>,
    Json(payload): Json<create_category_request>,
) -> Result<Json<Category>, StatusCode> {
    let category_id: i32 = sqlx::query_scalar!(
        "INSERT INTO categories (name, parent_id) VALUES ($1, $2) RETURNING id",
        payload.name,
        payload.parent_id.unwrap() as i64
    )
        .fetch_one(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to insert new category: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let category = Category{
        id: category_id as i32,
        name: payload.name,
        parent_id: payload.parent_id,
    };

    Ok(Json(category))
}