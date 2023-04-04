//use serde::Deserializer;
//use std::convert::From;

use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::types::Json;
use sqlx::types::JsonValue;

use crate::chrome::window::Window;

#[derive(Serialize, Default)]
//#[serde(tag = "type")]
pub enum Session {
    #[default]
    SavedSession,
    CurrentSession,
    PreviousSession,
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

fn default_deleted() -> String {
    "false".to_string()
}

//impl<'de> Deserialize<'de> for Session {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//    where
//        D: Deserializer<'de>,
//    {
//        #[derive(Debug, Deserialize)]
//        struct Mapping {
//            #[serde(rename = "type")]
//            type_: String
//        }
//
//        eprintln!("deserializer: {}", deserializer);
//
//        let Mapping { type_ } = Mapping::deserialize(deserializer)?;
//
//        match type_.as_str() {
//            "saved" => {
//                eprintln!("saved!");
//                Ok(Session::SavedSession{
//                })
//            }
//            "current" => {
//                eprintln!("saved!");
//                Ok(Session::CurrentSession{
//                })
//            }
//            "previous" => {
//                eprintln!("saved!");
//                Ok(Session::PreviousSession{
//                })
//            }
//            &_ => {
//                eprintln!("unknown!");
//                Ok(Session::PreviousSession{
//                })
//            }
//        }
//    }
//}

#[derive(sqlx::FromRow, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
pub struct SavedSession {
    // Key "id" is automatically incremented by SQLite. When running INSERT statements on the
    // database, skip this field.
    //pub id: i32,

    // FIXME CurrentSession has no id, so make this optional for now and later implement
    // CurrentSession and have some custom deserializer. (Session Buddy's datamodel )
    pub id: Option<i32>,

    // Key "gid" is present on SavedSessions in JSON exports but not in the database. Skip when
    // running INSERT statements on the database.
    #[sqlx(default)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,

    // Key "type" is present on SavedSession in JSON exports but not in the database. Actually ok.
    #[sqlx(default, rename = "type")]
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(default, skip_serializing_if = "is_default")]
    pub name: String,

    #[serde(default, skip_serializing_if = "is_default", rename = "generated")]
    pub generation_date_time: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "is_default", rename = "created")]
    pub creation_date_time: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "is_default", rename = "modified")]
    pub modification_date_time: DateTime<Utc>,

    #[serde(default, skip_serializing_if = "is_default")]
    pub tags: String,

    // users (IGNORED)

    // Column "deleted" has type TEXT in SQLite. Not
    #[serde(
        default = "default_deleted",
        //skip_serializing_if = "is_default"
    )]
    pub deleted: String,

    // thumbnail (IGNORED)

    // Actually JSON, serialization done by serde_json.
    pub windows: Json<Vec<Window>>,

    #[serde(default)]
    pub unfiltered_window_count: i32,

    #[serde(default)]
    pub filtered_window_count: i32,

    #[serde(default)]
    pub unfiltered_tab_count: i32,

    #[serde(default)]
    pub filtered_tab_count: i32
}

//impl From<&Path> for Vec<SavedSession> {
//    fn from(s: &Path) {
//    }
//}

impl SavedSession {
    // TODO Implement functions for filtered window/tab count

    pub fn count_windows(&self) -> i32 {
        self.windows.len().try_into().expect("USIZE->I32 FAILURE")
    }

    pub fn count_tabs(&self) -> i32 {
        self.windows
            .iter()
            .map(|w| {
                w.tabs
                    .as_ref()
                    .unwrap()
                    .len()
                    .try_into()
                    .expect("USIZE->I32 FAILURE")
            })
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>()
    }
}

impl From<&JsonValue> for SavedSession {
    fn from(s: &JsonValue) -> Self {
        serde_json::from_str(&s.to_string()).unwrap()
    }
}

#[derive(sqlx::FromRow, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
pub struct PreviousSession {
    id: i32,
    recording_date_time: DateTime<Utc>,
    creation_date_time: DateTime<Utc>,
    users: String,
    deleted: String,
    thumbnail: String,
    windows: Json<Vec<Window>>,
    unfiltered_window_count: i32,
    filtered_window_count: i32,
    unfiltered_tab_count: i32,
    filtered_tab_count: i32
}
