use serde::ser::StdError;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use crate::session_buddy::get_language;
use crate::session_buddy::get_platform;
use crate::session_buddy::get_user_agent;
use crate::session_buddy::session::get_previous_sessions;
use crate::session_buddy::session::get_saved_sessions;
use crate::session_buddy::session::Session;
use crate::session_buddy::session::Sessions;
use crate::session_buddy::settings::get_datetime_value_setting;
use crate::session_buddy::settings::get_string_value_setting;
use crate::session_buddy::SESSION_BUDDY_APPID;
use crate::session_buddy::SESSION_BUDDY_FORMAT;
use crate::session_buddy::SESSION_BUDDY_VERSION;

#[derive(Default, Debug, Serialize, Deserialize)]
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
    pub sessions: Vec<Session>,
}

impl Backup {
    pub async fn new(db: &Path, limit: Option<i64>) -> Result<Self, Box<dyn StdError>> {
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
            sb_installation_id: get_string_value_setting(db, "installationID").await?,
            sb_installed: get_datetime_value_setting(db, "installationTimeStamp").await?,
            sessions: vec![],
        };

        let _ = &b.collect(db, limit).await?;

        Ok(b)
    }

    pub async fn collect(
        &mut self,
        db: &Path,
        limit: Option<i64>,
    ) -> Result<(), Box<dyn StdError>> {
        self.sessions.extend(
            get_previous_sessions(db, limit)
                .await?
                .iter()
                .map(Session::from)
                .collect::<Sessions>(),
        );

        self.sessions.extend(
            get_saved_sessions(db, limit)
                .await?
                .iter()
                .map(Session::from)
                .collect::<Sessions>(),
        );

        Ok(())
    }
}
