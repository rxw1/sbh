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
use crate::util::generate_gid;

pub type Sessions = Vec<Session>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Session {
    #[serde(rename = "type")]
    pub type_: String,
    pub generated: DateTime<Utc>,

    //created: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<DateTime<Utc>>,

    // CurrentSession has no id or gid, only PreviousSession and SavedSession do.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    pub windows: Json<Windows> //pub windows: Vec<Window>,
}

// sqlite>
// PRAGMA table_info(SavedSessions);
//  0   id                     INTEGER  0  1
//  1   name                   TEXT     0  0
//  2   generationDateTime     NUMERIC  0  0
//  3   creationDateTime       NUMERIC  0  0
//  4   modificationDateTime   NUMERIC  0  0
//  5   tags                   TEXT     0  0
//  6   users                  TEXT     0  0
//  7   deleted                TEXT     0  0
//  8   thumbnail              TEXT     0  0
//  9   windows                TEXT     0  0
//  10  unfilteredWindowCount  INTEGER  0  0
//  11  filteredWindowCount    INTEGER  0  0
//  12  unfilteredTabCount     INTEGER  0  0
//  13  filteredTabCount       INTEGER  0  0

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Default)]
#[sqlx(rename_all = "camelCase")]
pub struct SavedSession {
    pub id: i64,
    pub name: String,
    pub generation_date_time: DateTime<Utc>,
    pub creation_date_time: DateTime<Utc>,
    pub modification_date_time: DateTime<Utc>,
    pub tags: String,
    pub users: String,
    pub deleted: String,
    pub thumbnail: String,
    pub windows: Json<Windows>,
    pub unfiltered_window_count: i64,
    pub filtered_window_count: i64,
    pub unfiltered_tab_count: i64,
    pub filtered_tab_count: i64
}

impl From<&Session> for SavedSession {
    fn from(s: &Session) -> Self {
        let windows = s.windows.clone(); // TODO
        SavedSession {
            //type_: "saved".to_string(),
            generation_date_time: s.generated,
            modification_date_time: s.modified.unwrap(),
            id: s.id.unwrap(),
            //thumbnail: s.thumbnail,
            windows,
            ..Default::default()
        }
    }
}


impl From<&SavedSession> for Session {
    fn from(s: &SavedSession) -> Self {
        let windows = s.windows.clone(); // TODO
        Session {
            type_: "saved".to_string(),
            generated: s.generation_date_time,
            //created: s.creation_date_time,
            modified: Some(s.modification_date_time),
            id: Some(s.id),
            gid: Some(generate_gid()),
            windows
        }
    }
}

// sqlite>
// PRAGMA table_info(PreviousSessions);
//  0   id                     INTEGER  0  1
//  1   recordingDateTime      NUMERIC  0  0
//  2   creationDateTime       NUMERIC  0  0
//  3   users                  TEXT     0  0
//  4   deleted                TEXT     0  0
//  5   thumbnail              TEXT     0  0
//  6   windows                TEXT     0  0
//  7   unfilteredWindowCount  INTEGER  0  0
//  8   filteredWindowCount    INTEGER  0  0
//  9   unfilteredTabCount     INTEGER  0  0
//  10  filteredTabCount       INTEGER  0  0

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
    filtered_tab_count: i64
}

impl From<&PreviousSession> for Session {
    fn from(s: &PreviousSession) -> Self {
        let windows = s.windows.clone(); // TODO
        Session {
            type_: "previous".to_string(),
            generated: s.recording_date_time,
            //created: s.creation_date_time,
            modified: None,
            id: Some(s.id),
            gid: Some(generate_gid()),
            windows
        }
    }
}

// TODO Dry up

pub async fn get_saved_sessions(db: &Path) -> Result<Vec<SavedSession>, Box<dyn StdError>> {
    if !db.exists() {
        eprintln!("Database file not found");
        std::process::exit(1)
    }

    let mut conn = SqliteConnection::connect(db.to_str().expect("[102] BAD DB PATH")).await?;

    Ok(
        sqlx::query_as::<_, SavedSession>("SELECT * FROM SavedSessions")
            .fetch_all(&mut conn)
            .await?
    )
}

pub async fn get_previous_sessions(db: &Path) -> Result<Vec<PreviousSession>, Box<dyn StdError>> {
    if !db.exists() {
        eprintln!("Database file not found");
        std::process::exit(1)
    }

    let mut conn = SqliteConnection::connect(db.to_str().expect("[382] BAD DB PATH")).await?;

    Ok(
        sqlx::query_as::<_, PreviousSession>("SELECT * FROM PreviousSessions")
            .fetch_all(&mut conn)
            .await?
    )
}
