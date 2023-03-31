use sqlx::Connection;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::SqliteConnection;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// UserSettings may have no fields at all.
pub struct UserSettings {
    #[serde(
        rename = "sessionExport_Format",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_export_format: Option<String>,

    #[serde(
        rename = "sessionExport_Scope",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_export_scope: Option<String>,

    #[serde(
        rename = "sessionExport_ShowTitles",
        skip_serializing_if = "Option::is_none"
    )]
    pub session_export_show_titles: Option<String>,

    #[serde(
        rename = "sessionExport_ShowURLs",
        skip_serializing_if = "Option::is_none",
    )]
    pub session_export_show_urls: Option<String>,
}

// skip_serializing_if = "<[_]>::is_empty"
// skip_serializing_if = "Map::is_empty"
// skip_serializing_if = "Vec::is_empty"

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
struct Setting {
    key: String,
    value: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct StringValue {
    value: String,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
struct DateTimeValue {
    value: DateTime<Utc>,
}

// TODO
pub async fn get_string_value_setting(
    db: &Path,
    table: &str,
    name: &str,
) -> Result<String, sqlx::Error> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;
    let q = format!("SELECT value FROM {} WHERE key = ?", table);
    Ok(sqlx::query_as::<_, StringValue>(&q)
        .bind(name)
        .fetch_one(&mut conn)
        .await?
        .value)
}

// TODO
pub async fn get_datetime_value_setting(
    db: &Path,
    table: &str,
    name: &str,
) -> Result<DateTime<Utc>, sqlx::Error> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;
    let q = format!("SELECT value FROM {} WHERE key = ?", table);
    Ok(sqlx::query_as::<_, DateTimeValue>(&q)
        .bind(name)
        .fetch_one(&mut conn)
        .await?
        .value)
}
