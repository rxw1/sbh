use std::error::Error;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;
use globwalk::DirEntry;
use serde::ser::StdError;
use serde::Deserialize;
use serde::Serialize;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::types::Json;
use sqlx::Connection;
use sqlx::SqliteConnection;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Limit rows
    #[arg(short)]
    limit: Option<i16>,

    /// Session Buddy databases
    #[arg(trailing_var_arg = true)]
    databases: Vec<String>,
}

type Windows = Vec<Window>;
type Tabs = Vec<Tab>;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
#[sqlx(rename_all = "camelCase")]
struct SavedSession {
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

#[derive(Debug, Serialize, Deserialize)]
struct Window {
    #[serde(default)]
    nx_title: String,
    state: String,
    #[serde(rename = "type")]
    type_: String,
    tabs: Json<Tabs>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tab {
    title: String,
    url: String,
    #[serde(default)]
    fav_icon_url: String,
    pinned: bool,
}

fn find_databases(basedir: &PathBuf) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    const APPID: &str = "edacconmaakjimmfgnblocblbcdcpbko";
    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        basedir,
        &[format!(
            "/*/*/User Data/Default/databases/chrome-extension_{}_*/*",
            APPID
        )],
    )
    .max_depth(8) // min 7
    .follow_links(true)
    .build()
    .unwrap()
    .filter_map(Result::ok);

    Ok(walker.collect::<Vec<DirEntry>>())
}

fn get_path() -> Option<PathBuf> {
    match whoami::platform() {
        whoami::Platform::Linux => match wsl::is_wsl() {
            true => Some(PathBuf::from(format!(
                "/mnt/c/Users/{}/AppData/Local",
                whoami::realname()
            ))),
            false => dirs::config_dir(),
        },
        whoami::Platform::MacOS => dirs::data_local_dir(),
        whoami::Platform::Windows => dirs::data_local_dir(),
        _ => {
            eprintln!("Operating system not supported");
            exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();

    let dbs = match args.databases.is_empty() {
        true => {
            let p = get_path().unwrap();
            find_databases(&p)
                .unwrap()
                .iter()
                .map(|x| PathBuf::from(x.path()))
                .collect::<Vec<PathBuf>>()
        }
        false => args
            .databases
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>(),
    };

    for db in dbs.iter() {
        let mut conn = SqliteConnection::connect(db.as_path().to_str().unwrap()).await?;

        let rows = match args.limit {
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

        println!("{}", serde_json::to_string(&rows).unwrap());
    }
    Ok(())
}
