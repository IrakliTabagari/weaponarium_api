use anyhow::{Context, Result};
use sqlx::{Postgres, Transaction};

const SQL: &str = include_str!("sql/seed_category.sql");

pub async fn seed(tx: &mut Transaction<'_, Postgres>) -> Result<()> {
    let categories: &[(&str, Option<i32>)] =
        &[("Weapons", None), ("Armor", None), ("Accessories", None)];

    for &(name, parent_id) in categories {
        sqlx::query(SQL)
            .bind(name)
            .bind(parent_id)
            .execute(&mut **tx)
            .await
            .with_context(|| format!("Failed to seed category '{name}'"))?;
    }

    Ok(())
}
