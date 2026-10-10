mod categories;
mod permissions;
mod roles;

use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is missing")?;

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;

    let mut tx = pool.begin().await?;

    println!("Seeding categories...");
    categories::seed(&mut tx)
        .await
        .context("Category seeding failed")?;

    //     println!("Seeding roles...");
    //     roles::seed(&mut tx).await.context("Role seeding failed")?;
    //
    //     println!("Seeding permissions...");
    //     permissions::seed(&mut tx)
    //         .await
    //         .context("Permission seeding failed")?;

    tx.commit().await.context("Failed to commit seed data")?;

    println!("Seeding completed.");
    pool.close().await;

    Ok(())
}
