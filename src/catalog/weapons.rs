use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize)]
pub struct create_weapon_request {
    name: String,
    category_id: i32
}

pub async fn create_weapon(
    State(pool): State<PgPool>,
    Json(payload): Json<create_weapon_request>,
) -> Result<Json<Weapon>, StatusCode> {
    let category_id: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM categories WHERE id = $1"
    )
        .bind(payload.category_id)
        .fetch_optional(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to fetch category: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if category_id.is_none() {
        eprintln!("invalid category_id");
        return Err(
            StatusCode::BAD_REQUEST
        );
    }


    let weapon_id: i32 = sqlx::query_scalar!(
        "INSERT INTO weapons (name, category_id) VALUES ($1, $2) RETURNING id",
        payload.name,
        payload.category_id as i64
    )
        .fetch_one(&pool)
        .await
        .map_err(|error| {
            eprintln!("Failed to insert new weapon: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(Weapon {
        id: weapon_id,
        name: payload.name,
        category_id: payload.category_id
    }))
}