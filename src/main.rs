use clap::Parser;
use env_logger::Env;
use serde::ser::StdError;

use log::{error, info};
use sbh::args::{Action, Args, ValidateAction};
use sbh::session_buddy::{backup, database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let env = Env::default().default_filter_or("info");

    env_logger::Builder::from_env(env)
        //.format(|buf, record| {
        //    writeln!(buf, "{}: {}", record.level(), record.args())
        //})
        .format_timestamp(None)
        .init();

    // TODO This could be cleaned up a little bit more

    let args = Args::parse();
    match args.action {
        Action::Dump { path } => {
            database::dump(&path).await.unwrap_or_else(|e| {
                error!("{:?}", e);
                std::process::exit(1)
            });
        }

        Action::Stats { path } => {
            database::stats(&path).await.unwrap_or_else(|e| {
                error!("{:?}", e);
                std::process::exit(1)
            });
        }

        Action::Id { path } => match database::id(&path).await {
            Ok(id) => println!("{}", id),
            Err(e) => {
                error!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::Backup { path, out } => {
            // TODO the search option operates on multiple databases, b/c
            // multiple may be found via searching. This is
            // inconsistent with other behaviour. Make a decision.

            // TODO Remove the clones().

            //if search {
            //    let paths = database::search(None).await?;

            //    for path in paths {
            //        database::backup(&path, out.clone())
            //            .await
            //            .unwrap_or_else(|e| {
            //                error!("{:?}", e);
            //                std::process::exit(1)
            //            });
            //    }
            //} else {
            database::backup(&path, out.clone())
                .await
                .unwrap_or_else(|e| {
                    error!("{:?}", e);
                    std::process::exit(1)
                });
            //}
        }

        Action::New { path } => {
            database::create(&path).await.unwrap_or_else(|e| {
                error!("{:?}", e);
                std::process::exit(1)
            });
        }

        Action::Validate { action } => match action {
            ValidateAction::Database { path } => match database::validate(&path).await {
                Ok(_) => {
                    info!("Database valid");
                }
                Err(e) => {
                    error!("{:?}", e);
                    std::process::exit(1)
                }
            },
            ValidateAction::Backup { path } => match backup::validate(&path).await {
                Ok(_) => {
                    info!("Backup valid");
                }
                Err(e) => {
                    error!("{:?}", e);
                    std::process::exit(1)
                }
            }
        },

        Action::Search { path } => {
            let t = sbh::util::ts();
            match database::search(path).await {
                Ok(dbs) => {
                    info!("Search took {:?}", sbh::util::ts() - t);
                    info!("Databases found: {}", dbs.len());
                    for db in dbs.iter() {
                        println!("{}", db.display());
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
        }
    }

    Ok(())
}
