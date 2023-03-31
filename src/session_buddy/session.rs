use std::convert::From;
use std::path::Path;

use serde::ser::StdError;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::types::Json;
use sqlx::Connection;
use sqlx::SqliteConnection;

use crate::chrome::window::Windows;
use crate::session_buddy::generate_gid;

pub type Sessions = Vec<Session>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "type")]
    type_: String,
    generated: DateTime<Utc>,
    created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<DateTime<Utc>>,
    id: i64,
    gid: String,
    windows: Json<Windows>,
}

impl From<&SavedSession> for Session {
    fn from(s: &SavedSession) -> Self {
        let windows = s.windows.clone(); // TODO
        Session {
            type_: "saved".to_string(),
            generated: s.generation_date_time,
            created: s.creation_date_time,
            modified: Some(s.modification_date_time),
            id: s.id,
            gid: generate_gid(),
            windows,
        }
    }
}

impl From<&PreviousSession> for Session {
    fn from(s: &PreviousSession) -> Self {
        let windows = s.windows.clone(); // TODO
        Session {
            type_: "previous".to_string(),
            generated: s.recording_date_time,
            created: s.creation_date_time,
            modified: None,
            id: s.id,
            gid: generate_gid(),
            windows,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
pub struct SavedSession {
    id: i64,
    name: String,
    generation_date_time: DateTime<Utc>,
    creation_date_time: DateTime<Utc>,
    modification_date_time: DateTime<Utc>,
    tags: String,
    users: String,
    deleted: String,
    thumbnail: String,
    windows: Json<Windows>,
    unfiltered_window_count: i64,
    filtered_window_count: i64,
    unfiltered_tab_count: i64,
    filtered_tab_count: i64,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
pub struct PreviousSession {
    id: i64,
    recording_date_time: DateTime<Utc>,
    creation_date_time: DateTime<Utc>,
    users: String,
    deleted: String,
    thumbnail: String,
    windows: Json<Windows>,
    unfiltered_window_count: i64,
    filtered_window_count: i64,
    unfiltered_tab_count: i64,
    filtered_tab_count: i64,
}

pub async fn get_saved_sessions(
    db: &Path,
    limit: Option<i64>,
) -> Result<Vec<SavedSession>, Box<dyn StdError>> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;
    let rows = match limit {
        Some(limit) => {
            sqlx::query_as::<_, SavedSession>("SELECT * FROM SavedSessions LIMIT ?")
                .bind(limit)
                .fetch_all(&mut conn)
                .await?
        }
        None => {
            sqlx::query_as::<_, SavedSession>("SELECT * FROM SavedSessions")
                .fetch_all(&mut conn)
                .await?
        }
    };
    Ok(rows)
}

pub async fn get_previous_sessions(
    db: &Path,
    limit: Option<i64>,
) -> Result<Vec<PreviousSession>, Box<dyn StdError>> {
    let mut conn = SqliteConnection::connect(db.to_str().unwrap()).await?;
    let rows = match limit {
        Some(limit) => {
            sqlx::query_as::<_, PreviousSession>("SELECT * FROM PreviousSessions LIMIT ?")
                .bind(limit)
                .fetch_all(&mut conn)
                .await?
        }
        None => {
            sqlx::query_as::<_, PreviousSession>("SELECT * FROM PreviousSessions")
                .fetch_all(&mut conn)
                .await?
        }
    };
    Ok(rows)
}
