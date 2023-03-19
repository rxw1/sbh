use sqlx::Connection;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::SqliteConnection;

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

pub async fn get_string_value_setting(db: &Path, name: &str) -> Result<String, sqlx::Error> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;

    Ok(
        sqlx::query_as::<_, StringValue>("SELECT value FROM Settings WHERE key = ?")
            .bind(name)
            .fetch_one(&mut conn)
            .await?
            .value,
    )
}

pub async fn get_datetime_value_setting(
    db: &Path,
    name: &str,
) -> Result<DateTime<Utc>, sqlx::Error> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;

    Ok(
        sqlx::query_as::<_, DateTimeValue>("SELECT value FROM Settings WHERE key = ?")
            .bind(name)
            .fetch_one(&mut conn)
            .await?
            .value,
    )
}
