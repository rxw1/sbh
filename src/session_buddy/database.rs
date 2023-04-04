use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use log::info;
use serde::ser::StdError;
use sqlx::sqlite::{SqliteConnectOptions, SqliteQueryResult};
use sqlx::{ConnectOptions, SqliteConnection};

use crate::chrome::get_path;
use crate::session_buddy::settings::{get_datetime_value_setting, get_string_value_setting};
use crate::util::get_output_filename;

use super::backup::Backup;
use super::session::{PreviousSession, SavedSession};

// Key "tags" is present on SavedSessions in the database
// but not in JSON backups. Not ok.

#[derive(Default)]
pub struct Stats {
    pub sessions: i32,
    pub tabs: i32,
    pub windows: i32,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub atime: DateTime<Utc>,
    pub mtime: DateTime<Utc>,
    pub ctime: DateTime<Utc>,
    pub newest_session: DateTime<Utc>,
    pub oldest_session: DateTime<Utc>,
    pub duplicate_urls: i32
}

//struct SessionStats {}
//struct TabStats {}

/// Get a new connection to a SQLite database.
pub async fn connect(filename: &Path) -> Result<SqliteConnection, sqlx::Error> {
    SqliteConnectOptions::from_str(format!("sqlite://{}", filename.display()).as_str())?
        .log_statements(log::LevelFilter::Trace)
        .create_if_missing(false)
        .connect()
        .await
}

/// Create a new SQLite database with the Session Buddy
/// schema.
pub async fn create(filename: &Path) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut conn =
        SqliteConnectOptions::from_str(format!("sqlite://{}", filename.display()).as_str())?
            .log_statements(log::LevelFilter::Trace)
            .create_if_missing(true)
            .connect()
            .await?;

    sqlx::query(SCHEMA).execute(&mut conn).await
}

/// Insert a serialized SavedSession into the database.
pub async fn insert_saved_session(
    conn: &mut SqliteConnection,
    session: &SavedSession
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO SavedSessions (
            -- id -- AUTOINCREMENTED
            name,
            generationDateTime,
            creationDateTime,
            modificationDateTime,
            tags,
            -- users -- IGNORED
            deleted,
            -- thumbnail -- IGNORED
            windows,
            unfilteredWindowCount,
            filteredWindowCount,
            unfilteredTabCount,
            filteredTabCount
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#
    )
    .bind(&session.name)
    .bind(session.generation_date_time)
    .bind(session.creation_date_time)
    .bind(session.modification_date_time)
    .bind(&session.tags)
    .bind(&session.deleted)
    .bind(&session.windows)
    .bind(session.count_windows())
    .bind(session.count_windows())
    .bind(session.count_tabs())
    .bind(session.count_tabs())
    .execute(conn)
    .await
}

/// Gets all SavedSessions from Session Buddy JSON exports
/// and returns them in a Vec<SavedSession>.
pub async fn collect_saved_sessions(
    files: &[PathBuf]
) -> Result<Vec<SavedSession>, std::io::Error> {
    let mut saved_sessions: Vec<SavedSession> = vec![];

    for file in files.iter() {
        let j: serde_json::Value =
            serde_json::from_str(fs::read_to_string(file)?.trim_start_matches('\u{feff}'))?;

        if let Some(sessions) = j.get("sessions") {
            saved_sessions.append(
                &mut sessions
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter(|x| x.get("type").expect("SESSION HAS NO TYPE KEY") != "current")
                    .map(SavedSession::from)
                    .collect::<Vec<SavedSession>>()
            );
        }
    }

    Ok(saved_sessions)
}

pub async fn import(
    conn: &mut SqliteConnection,
    saved_sessions: &Vec<SavedSession>
) -> Result<(), sqlx::Error> {
    for session in saved_sessions {
        insert_saved_session(conn, session).await?;
    }
    Ok(())
}

pub fn copy_db(db: &Path, out: PathBuf) -> Result<(), std::io::Error> {
    let o = get_output_filename(db, Some(out))?;
    fs::copy(db, o)?;
    Ok(())
}

/// Run `PRAGMA integrity_check` and import the Database
/// into a struct. If that succeeds, the
/// database can be considered ok.
pub async fn validate(path: &Path) -> Result<(), Box<dyn StdError>> {
    let mut conn = connect(path).await?;
    sqlx::query("PRAGMA integrity_check")
        .execute(&mut conn)
        .await?;

    // TODO We need a custom deserializer to handle all SavedSession,
    // PreviousSession and CurrentSession as an enum
    // Session::SavedSession etc. (Or just not to care about that would
    // be another option). Who cares about PreviousSession and
    // CurrentSession? They are automatically generated anyway.
    //let backup = Backup::new(path).await?;
    //serde_json::to_string(&backup)?;

    Ok(())
}

/// Export a Session Buddy database to a JSON file, similar
/// to what the extension would produce. The file should be
/// fit to be imported into a database again.
pub async fn backup(db: &Path, out: Option<PathBuf>) -> Result<(), Box<dyn StdError>> {
    let backup = Backup::new(db).await?;
    if out.is_some() {
        let fallback = PathBuf::from(".");
        let what = out.unwrap_or(fallback);
        let file: File = File::create(what)?; // TODO
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &backup)?;
        writer.flush()?;
    } else {
        // Print JSON to stdout
        println!("{}", serde_json::to_string(&backup).unwrap());
    }
    Ok(())
}

/// Traverse a fs path to find Session Buddy databases.
pub async fn search(basepath: Option<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // If no path is given, first try to figure out a platform
    // dependend path, and if that fails default to the
    // current working path.
    let base = &basepath.unwrap_or_else(|| get_path().unwrap_or(".".into()));

    info!("Searching {}", base.display());

    // TODO This is not very reliable
    if wsl::is_wsl() && base.starts_with("/mnt/c") {
        // It's known that doing fs operations across WSL2 and
        // Windows is slow, but still this feels a lot
        // slower than what I'm already expecting. (Something
        // between 40 seconds and 4 minutes!)
        info!(
            "{} {} {}",
            "Operating on Windows from WSL2 may take some time.",
            "Please be patient.",
            "Alternatively specify a path to search."
        );
    }

    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        base,
        &[format!(
            "/*/*/User Data/Default/databases/chrome-extension_{}_*/*",
            crate::session_buddy::SESSION_BUDDY_APPID
        )]
    )
    .max_depth(crate::SBH_MAX_WALK_DEPTH) // min 7
    .follow_links(true)
    .build()?
    .filter_map(Result::ok);

    let dbs = walker
        .map(|x| PathBuf::from(x.path()))
        .collect::<Vec<PathBuf>>();

    Ok(dbs)
}

pub async fn saved_sessions(db: &Path) -> Result<Vec<SavedSession>, Box<dyn StdError>> {
    let mut conn = connect(db).await?;

    Ok(
        sqlx::query_as::<_, SavedSession>("SELECT * FROM SavedSessions")
            .fetch_all(&mut conn)
            .await?
    )
}

pub async fn previous_sessions(db: &Path) -> Result<Vec<PreviousSession>, Box<dyn StdError>> {
    let mut conn = connect(db).await?;

    Ok(
        sqlx::query_as::<_, PreviousSession>("SELECT * FROM PreviousSessions")
            .fetch_all(&mut conn)
            .await?
    )
}

/// Print all URLs to stdout
pub async fn dump(path: &Path) -> Result<(), Box<dyn StdError>> {
    for session in saved_sessions(path).await? {
        for window in session.windows.iter() {
            match &window.tabs {
                Some(tabs) => {
                    for tab in tabs.iter() {
                        match &tab.url {
                            Some(url) => {
                                println!("{}", url);
                            }
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
    }
    // TODO Refactor, this is stupid
    Ok(())
}

pub async fn id(path: &Path) -> Result<String, sqlx::Error> {
    get_string_value_setting(path, "Settings", "installationID").await
}

pub async fn stats(path: &Path) -> Result<(), Box<dyn StdError>> {
    let installation_id = get_string_value_setting(path, "Settings", "installationID").await?;

    let installation_date =
        get_datetime_value_setting(path, "Settings", "installationTimeStamp").await?;

    let sessions = saved_sessions(path).await?;

    let session_count = sessions.len();

    let window_count: i32 = sessions.iter().map(|s| s.count_windows()).sum();

    let tab_count: i32 = sessions.iter().map(|s| s.count_tabs()).sum();

    println!("Path:              {}", path.display());
    println!("Installation ID:   {}", installation_id);
    println!("Installation Date: {}", installation_date);
    println!("Tabs:              {:>5}", tab_count);
    println!("Windows:           {:>5}", window_count);
    println!("Sessions:          {:>5}", session_count);

    Ok(())
}

const SCHEMA: &str = r#"
CREATE TABLE Settings (
    key TEXT PRIMARY KEY,
    value NUMERIC);

CREATE TABLE UserSettings (
    key TEXT PRIMARY KEY,
    value NUMERIC);

CREATE TABLE Undo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    creationDateTime NUMERIC,
    tabIdentifier TEXT,
    action TEXT,
    description TEXT,
    register1 TEXT,
    register2 TEXT,
    register3 TEXT,
    register4 TEXT,
    register5 TEXT);

CREATE TABLE SavedSessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    generationDateTime NUMERIC,
    creationDateTime NUMERIC,
    modificationDateTime NUMERIC,
    tags TEXT,
    users TEXT,
    deleted TEXT,
    thumbnail TEXT,
    windows TEXT,
    unfilteredWindowCount INTEGER,
    filteredWindowCount INTEGER,
    unfilteredTabCount INTEGER,
    filteredTabCount INTEGER);

CREATE TABLE PreviousSessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recordingDateTime NUMERIC,
    creationDateTime NUMERIC,
    users TEXT,
    deleted TEXT,
    thumbnail TEXT,
    windows TEXT,
    unfilteredWindowCount INTEGER,
    filteredWindowCount INTEGER,
    unfilteredTabCount INTEGER,
    filteredTabCount INTEGER);
"#;
