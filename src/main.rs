use clap::Parser;
use serde::ser::StdError;

use log::{error, info};
use sbh::args::{Action, Args, DebugAction};
use sbh::session_buddy::database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    pretty_env_logger::init();
    let args = Args::parse();
    match args.action {
        Action::Backup { db, out } => match database::backup(&db, out).await {
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

        Action::Import { database, files } => {
            match database::collect_saved_sessions(&files).await {
                Ok(sessions) => {
                    let mut conn = database::connect(&database).await?;
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
            }
        }

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
