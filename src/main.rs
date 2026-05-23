use anyhow::Result;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    // connect to the database
    let pool = SqlitePool::connect("sqlite://scanner.db").await?;

    // execute the schema creation
    sqlx::query!(
        "CREATE TABLE IF NOT EXISTS scans (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ip TEXT NULL,
            port INTEGER NULL NULL,
            status TEXT NOT NULL,
            scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    println!("Hello, world!");

    Ok(())
}
