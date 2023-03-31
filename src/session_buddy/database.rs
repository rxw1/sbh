use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use serde::ser::StdError;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::ConnectOptions;
use sqlx::Connection;
use sqlx::SqliteConnection;

use crate::chrome::get_path;
use crate::session_buddy::backup::Backup;
use crate::util::get_output_filename;

use super::session::SavedSession;

#[derive(Default)]
pub struct Database {}

pub async fn import(database: &Path, files: &[PathBuf]) -> Result<(), std::io::Error> {
    eprintln!("{:?}", database.display());
    for file in files.iter() {
        let j: serde_json::Value =
            serde_json::from_str(fs::read_to_string(file)?.trim_start_matches('\u{feff}'))?;
        let b: Backup = serde_json::from_str(&j.to_string()).unwrap();
        println!("{:#?}", b.sessions);
        for session in b.sessions.iter() {
            match session.type_.as_str() {
                "saved" => {
                    //let s: SavedSession = session.into();
                    //insert_saved_session(database, s);
                }
                "current" => {
                    eprintln!("current session")
                }
                _ => {
                    eprintln!("unknown session type")
                }
            }
        }
        //println!("{}",
        // serde_json::to_string(&b).unwrap());
    }
    Ok(())
}

pub fn copy_db(db: &Path, out: PathBuf) -> Result<(), std::io::Error> {
    let o = get_output_filename(db, Some(out))?;
    fs::copy(db, o)?;
    Ok(())
}

pub async fn insert_saved_session(
    filename: &Path,
    saved_session: SavedSession
) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut conn = SqliteConnection::connect(filename.as_os_str().to_str().unwrap()).await?; // TODO
    sqlx::query(r#"INSERT INTO SavedSessions VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#)
        .bind(&saved_session.id)
        .bind(&saved_session.name)
        .bind(&saved_session.generation_date_time)
        .bind(&saved_session.creation_date_time)
        .bind(&saved_session.modification_date_time)
        .bind(&saved_session.tags)
        .bind(&saved_session.users)
        .bind(&saved_session.deleted)
        .bind(&saved_session.thumbnail)
        .bind(&saved_session.windows)
        .bind(&saved_session.unfiltered_window_count)
        .bind(&saved_session.filtered_window_count)
        .bind(&saved_session.unfiltered_tab_count)
        .bind(&saved_session.filtered_tab_count)
        .execute(&mut conn)
        .await
}

/// Create a new database with bare minimum contents.
pub async fn create(filename: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let q = format!("sqlite://{}", filename);
    println!("Database::create {:?}", q);
    let mut conn = SqliteConnectOptions::from_str(q.as_str())?
        .create_if_missing(true)
        .connect()
        .await?;
    sqlx::query(SCHEMA).execute(&mut conn).await
}

/// Run `PRAGMA integrity_check` and import the Database
/// into a struct. If that succeeds, the
/// database can be considered ok.
pub async fn validate(filename: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let mut conn = SqliteConnection::connect(filename).await?;
    sqlx::query("PRAGMA integrity_check")
        .execute(&mut conn)
        .await
}

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
        println!("{}", serde_json::to_string(&backup).unwrap());
    }

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

/// Traverse a path to find Session Buddy databases.
pub async fn search(basepath: Option<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // If no path is given, first try to figure out a platform
    // dependend path, and if that fails default to the
    // current working path.
    let base = &basepath.unwrap_or_else(|| get_path().unwrap_or(".".into()));

    println!("Searching {}", base.display());

    // TODO This is not very reliable
    if wsl::is_wsl() && base.starts_with("/mnt/c") {
        // It's known that doing fs operations across WSL2 and
        // Windows is slow, but still this feels a lot
        // slower than what I'm already expecting. (Something
        // between 40 seconds and 4 minutes!)
        eprintln!(
            "{}",
            textwrap::fill(
                "Operating on Windows from WSL2 may take some time. Please be patient. \
                         Alternatively specify a path to search.",
                77
            )
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
