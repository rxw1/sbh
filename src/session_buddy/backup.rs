use std::path::Path;

use chrono::DateTime;
use chrono::Utc;
use serde::ser::StdError;
use serde::Deserialize;
use serde::Serialize;

use crate::session_buddy::database;
use crate::session_buddy::session::SavedSession;
use crate::session_buddy::settings::get_datetime_value_setting;
use crate::session_buddy::settings::get_string_value_setting;
use crate::session_buddy::settings::UserSettings;
use crate::session_buddy::SESSION_BUDDY_APPID;
use crate::session_buddy::SESSION_BUDDY_FORMAT;
use crate::session_buddy::SESSION_BUDDY_VERSION;
use crate::util::get_language;
use crate::util::get_platform;
use crate::util::get_user_agent;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Backup {
    pub format: String,
    pub created: DateTime<Utc>,
    pub session_scope: String,
    pub include_session: bool,
    pub include_window: bool,
    pub platform: String,
    pub language: String,
    pub ua: String,
    pub sb_id: String,
    pub sb_version: String,
    pub sb_installation_id: String,
    pub sb_installed: DateTime<Utc>,
    pub sessions: Vec<SavedSession>,
    pub user_settings: UserSettings
}

impl Backup {
    pub async fn new(db: &Path) -> Result<Self, Box<dyn StdError>> {
        let mut b = Backup {
            format: SESSION_BUDDY_FORMAT.to_string(),
            created: Utc::now(),
            session_scope: "all".to_string(),
            include_session: true,
            include_window: true,
            platform: get_platform(),
            language: get_language(),
            ua: get_user_agent(),
            sb_id: SESSION_BUDDY_APPID.to_string(),
            sb_version: SESSION_BUDDY_VERSION.to_string(),
            sb_installation_id: get_string_value_setting(db, "Settings", "installationID").await?,
            sb_installed: get_datetime_value_setting(db, "Settings", "installationTimeStamp")
                .await?,
            sessions: vec![],
            user_settings: UserSettings {
                ..Default::default()
            }
        };

        let _ = &b.collect(db).await?;

        Ok(b)
    }

    pub async fn collect(&mut self, db: &Path) -> Result<(), Box<dyn StdError>> {
        // Previous sessions
        //self.sessions.extend(
        //    get_previous_sessions(db)
        //        .await?
        //        .iter()
        //        .map(Session::from)
        //        .collect::<Vec<PreviousSession>>(),
        //);

        // Saved sessions
        self.sessions.extend(
            database::saved_sessions(db)
                .await?
                //.iter()
                //.map(SavedSession::from)
                //.collect::<Vec<SavedSession>>(),
        );

        // Don't care about the current session

        Ok(())
    }
}

/// TODO This could be done better.
pub async fn validate(path: &Path) -> Result<(), Box<dyn StdError>> {
    let _: Backup = serde_json::from_str(std::fs::read_to_string(path)?.trim_start_matches('\u{feff}'))?;
    Ok(())
}
