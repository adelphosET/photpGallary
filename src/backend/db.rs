// #[cfg(feature = "server")]
// use sqlx::{Executor, Pool, Sqlite};
// #[cfg(feature = "server")]
// use std::path::Path;
// #[cfg(feature = "server")]
// use tokio::sync::OnceCell;
// #[cfg(feature = "server")]
// use std::path::PathBuf;

// #[cfg(feature = "server")]
// static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

// #[cfg(feature = "server")]
// // async fn db() -> Pool<Sqlite> {
// //     // Database file path
// //     let db_path = "db.sqlite";

// //     // Ensure the directory exists (if you're using a path with directories)
// //     if let Some(parent) = Path::new(db_path).parent() {
// //         tokio::fs::create_dir_all(parent).await.unwrap();
// //     }

// //     // This connection will create the file if it doesn't exist
// //     let pool = sqlx::sqlite::SqlitePool::connect(&format!("sqlite://{}", db_path))
// //         .await
// //         .unwrap();

// //     pool.execute(
// //         "
// //           CREATE TABLE IF NOT EXISTS images (
// //               id INTEGER PRIMARY KEY AUTOINCREMENT,
// //               category TEXT,
// //               content BLOB,
// //               name TEXT
// //           );
// //           CREATE TABLE IF NOT EXISTS categories (
// //               id INTEGER PRIMARY KEY AUTOINCREMENT,
// //               name TEXT UNIQUE
// //           );
// //           ",
// //     )
// //     .await
// //     .unwrap();

// //     pool
// // }
// async fn db() -> Pool<Sqlite> {
//     // Build absolute path: <project_root>/data/db.sqlite
//     let db_path: PathBuf = std::env::current_dir()
//         .expect("Cannot get current directory")
//         .join("data")
//         .join("db.sqlite");

//     // Ensure parent directory exists
//     if let Some(parent) = db_path.parent() {
//         tokio::fs::create_dir_all(parent)
//             .await
//             .expect("Failed to create database directory");
//     }

//     // SQLite URL format with absolute path
//     let db_url = format!("sqlite://{}", db_path.display());

//     // Create/connect to DB
//     let pool = sqlx::sqlite::SqlitePool::connect(&db_url)
//         .await
//         .unwrap_or_else(|e| panic!("Failed to connect to SQLite at {db_url}: {e}"));

//     // Ensure schema exists
//     pool.execute(
//         r#"
//         CREATE TABLE IF NOT EXISTS images (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             category TEXT,
//             content BLOB,
//             name TEXT
//         );
//         CREATE TABLE IF NOT EXISTS categories (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name TEXT UNIQUE
//         );
//         "#,
//     )
//     .await
//     .expect("Failed to create tables");

//     pool
// }


// #[cfg(feature = "server")]
// pub async fn get_db() -> &'static Pool<Sqlite> {
//     DB.get_or_init(db).await
// }
#[cfg(feature = "server")]
use sqlx::{sqlite::SqliteConnectOptions, Executor, Pool, Sqlite};
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

    // Connect options with create_if_missing
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);

    // Create pool
    let pool = sqlx::SqlitePool::connect_with(options)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to SQLite at {}: {e}", db_path.display()));

    // Ensure schema exists
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
