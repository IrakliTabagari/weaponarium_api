use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct Category {
    id: i32,
    name: String,
    parent_id: Option<i32>,
}

pub async fn get_categories(State(pool): State<PgPool>) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories =
        sqlx::query_as::<_, Category>("SELECT id, name, parent_id FROM categories ORDER BY id")
            .fetch_all(&pool)
            .await
            .map_err(|error| {
                eprintln!("Failed to fetch categories: {error}");
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

    Ok(Json(categories))
}

#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    name: String,
    parent_id: Option<i32>,
}

pub async fn create_category(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateCategoryRequest>,
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

    let category = Category {
        id: category_id,
        name: payload.name,
        parent_id: payload.parent_id,
    };

    Ok(Json(category))
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: String,
    pub parent_id: Option<i32>,
}

pub async fn update_category(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<StatusCode, StatusCode> {
    let name = payload.name.trim();
    if name.is_empty() || name.chars().count() > 250 || payload.parent_id == Some(id) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, name, parent_id FROM categories WHERE id = $1 OR id = $2",
    )
    .bind(id)
    .bind(payload.parent_id)
    .fetch_all(&pool)
    .await
    .map_err(|error| {
        eprintln!("Failed to fetch categories: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !categories.iter().any(|category| category.id == id) {
        return Err(StatusCode::NOT_FOUND);
    }

    if let Some(parent_id) = payload.parent_id {
        if !categories.iter().any(|category| category.id == parent_id) {
            return Err(StatusCode::BAD_REQUEST);
        }

        // Walk up from the proposed parent. Finding this category means a cycle.
        // UNION also terminates if older data already contains a cycle.
        let creates_cycle: bool = sqlx::query_scalar(
            "WITH RECURSIVE ancestors AS (
                SELECT id, parent_id FROM categories WHERE id = $1
                UNION
                SELECT parent.id, parent.parent_id
                FROM categories AS parent
                JOIN ancestors ON parent.id = ancestors.parent_id
            )
            SELECT EXISTS (SELECT 1 FROM ancestors WHERE id = $2)",
        )
        .bind(parent_id)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to check category parents: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        if creates_cycle {
            return Err(StatusCode::BAD_REQUEST);
        }
    }

    let result = sqlx::query("UPDATE categories SET name = $1, parent_id = $2 WHERE id = $3")
        .bind(name)
        .bind(payload.parent_id)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to update category: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::OK)
}