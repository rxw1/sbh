use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
struct Undo {
    id: i64,
    creation_date_time: DateTime<Utc>,
    tab_identifier: String,
    action: String,
    description: String,
    register1: String,
    register2: String,
    register3: String,
    register4: String,
    register5: String
}
