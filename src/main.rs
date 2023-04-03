use clap::Parser;
use sbh::session_buddy::settings::get_datetime_value_setting;
use sbh::session_buddy::settings::get_string_value_setting;
use serde::ser::StdError;

use log::{error, info};
use sbh::args::{Action, Args, DebugAction};
use sbh::session_buddy::database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    pretty_env_logger::init();
    let args = Args::parse();
    match args.action {
        Action::Dump { path } => {
            for session in database::saved_sessions(&path).await? {
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
        }

        Action::Stats { path } => {
            let installation_id =
                get_string_value_setting(&path, "Settings", "installationID").await?;

            let installation_date =
                get_datetime_value_setting(&path, "Settings", "installationTimeStamp").await?;

            let sessions = database::saved_sessions(&path).await?;

            let session_count = sessions.len();

            let window_count: i32 = sessions.iter().map(|s| s.count_windows()).sum();

            let tab_count: i32 = sessions.iter().map(|s| s.count_tabs()).sum();

            println!("Path:              {}", path.display());
            println!("Installation ID:   {}", installation_id);
            println!("Installation Date: {}", installation_date);
            println!("Tabs:              {:>5}", tab_count);
            println!("Windows:           {:>5}", window_count);
            println!("Sessions:          {:>5}", session_count);
        }

        Action::Backup { path, out } => match database::backup(&path, out).await {
            Ok(_) => {
                info!("Database created");
            }
            Err(e) => {
                error!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::Search { path } => {
            let t = sbh::util::ts();
            match database::search(path).await {
                Ok(dbs) => {
                    info!("Search took {:?}", sbh::util::ts() - t);
                    info!("Databases found: {}", dbs.len());
                    for db in dbs.iter() {
                        info!("{}", db.display());
                    }
                }
                Err(e) => {
                    error!("{:?}", e);
                    std::process::exit(1)
                }
            }
        }

        Action::Import { path, files } => match database::collect_saved_sessions(&files).await {
            Ok(sessions) => {
                let mut conn = database::connect(&path).await?;
                match database::import(&mut conn, &sessions).await {
                    Ok(()) => {
                        info!("Imported {} sessions", sessions.len());
                    }
                    Err(e) => {
                        error!("{:?}", e);
                        std::process::exit(1)
                    }
                };
            }
            Err(e) => {
                error!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::New { path } => match database::create(&path).await {
            Ok(result) => {
                info!("Database created: {:?}", result);
            }
            Err(e) => {
                error!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::Debug { action } => match action {
            DebugAction::Database { path } => match database::validate(&path).await {
                Ok(_) => {
                    info!("Database valid");
                }
                Err(e) => {
                    error!("{:?}", e);
                    std::process::exit(1)
                }
            }
        }
    }

    Ok(())
}
