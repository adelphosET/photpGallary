#[cfg(feature = "server")]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct ImageDBSQL {
    pub id: i64,
    pub content: Vec<u8>,   // BLOB ↔ Vec<u8>
    pub name: String,
    pub category: String,
}



