////////////////////

#[cfg(feature = "server")]
use super::{db::get_db, model::ImageDBSQL};
use crate::app::ImageDB;
use dioxus::prelude::*;
#[server]
pub async fn insert_category(category: String) -> Result<i64, ServerFnError> {
    let db = get_db().await;
    let result = sqlx::query("INSERT OR IGNORE INTO categories (name) VALUES (?)")
        .bind(&category)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to insert category: {e}")))?;
    Ok(result.last_insert_rowid())
}

#[server]
pub async fn insert_image(
    name: String,
    category: String,
    image_data: Vec<u8>, // Binary input
) -> Result<i64, ServerFnError> {
    let db = get_db().await;
    let result =
        sqlx::query("INSERT INTO images (name, category, content) VALUES (?, ?, ?) RETURNING *")
            .bind(&name)
            .bind(&category)
            .bind(&image_data) // Binary binding
            .execute(db)
            .await
            .map_err(|_| ServerFnError::new("Failed to insert image"))?;
    Ok(result.last_insert_rowid())
}
#[server]
pub async fn get_all_images() -> Result<Vec<ImageDB>, ServerFnError> {
    let db = get_db().await;
    let rows: Vec<ImageDBSQL> = sqlx::query_as("SELECT * FROM images")
        .fetch_all(db)
        .await
        .unwrap();
    let mut img_stack = vec![];
    for row in rows {
        let all_images = ImageDB {
            id: Some(row.id),
            content: row.content,
            name: row.name,
            category: row.category,
        };
        img_stack.push(all_images);
    }
    Ok(img_stack)
}
#[server]
pub async fn get_only_sigle_img(category: String) -> Result<ImageDB, ServerFnError> {
    let db = get_db().await;
    let rows: Vec<ImageDBSQL> = sqlx::query_as("SELECT * FROM images WHERE category LIKE ?1")
        .bind(&category)
        .fetch_all(db)
        .await
        .unwrap();
    if rows.len() == 0 {
        let msg = format!("Todo category : {} Not Found.", category);
        Err(ServerFnError::new(msg))
    } else {
        let all_images = ImageDB {
            id: Some(rows[0].id.clone()),
            content: rows[0].content.clone(),
            name: rows[0].name.clone(),
            category: rows[0].category.clone(),
        };
        Ok(all_images)
    }
}

#[server]
pub async fn get_by_category(category: String) -> Result<Vec<ImageDB>, ServerFnError> {
    let db = get_db().await;
    let rows: Vec<ImageDBSQL> = sqlx::query_as("SELECT * FROM images WHERE category = ?1 LIMIT 12")
        .bind(&category)
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(format!("DB error: {e}")))?;

    if rows.is_empty() {
        return Err(ServerFnError::new(format!(
            "No images found for category: {category}"
        )));
    }

    let img_stack = rows
        .into_iter()
        .map(|row| ImageDB {
            id: Some(row.id),
            content: row.content,
            name: row.name,
            category: row.category,
        })
        .collect();

    Ok(img_stack)
}
