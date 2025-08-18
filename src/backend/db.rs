#[cfg(feature = "server")]
use sqlx::{Executor, Pool, Sqlite, sqlite::SqliteConnectOptions};
#[cfg(feature = "server")]
use std::path::PathBuf;
#[cfg(feature = "server")]
use tokio::sync::OnceCell;

#[cfg(feature = "server")]
static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

#[cfg(feature = "server")]
async fn db() -> Pool<Sqlite> {
    // Absolute path: <project_root>/data/db.sqlite
    let db_path: PathBuf = std::env::current_dir()
        .expect("Cannot get current directory")
        .join("data")
        .join("db.sqlite");

    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .expect("Failed to create database directory");
    }

    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .pragma("cache_size", "-10000")
        .pragma("foreign_keys", "ON")
        .pragma("busy_timeout", "5000")
        .pragma("temp_store", "MEMORY")
        .pragma("locking_mode", "NORMAL");

    let pool = sqlx::pool::PoolOptions::new()
        .max_connections(32) // SQLite is limited by file locking; 32 is a safe sweet spot
        .min_connections(4)
        .connect_with(options)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to SQLite at {}: {e}", db_path.display()));

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            category TEXT,
            content BLOB,
            name TEXT
        );
        CREATE TABLE IF NOT EXISTS categories (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE
        );                
        "#,
    )
    .await
    .expect("Failed to create tables");

    pool
}

#[cfg(feature = "server")]
pub async fn get_db() -> &'static Pool<Sqlite> {
    DB.get_or_init(db).await
}
