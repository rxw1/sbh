use clap::Parser;
use serde::ser::StdError;

use sbh::args::Action;
use sbh::args::Args;
use sbh::args::DebugAction;
use sbh::session_buddy::database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let args = Args::parse();
    match args.action {
        Action::Backup { db, out } => match database::backup(&db, out).await {
            Ok(_) => {
                println!("Database created");
            }
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::Search { path } => {
            let t = sbh::util::ts();
            match database::search(path).await {
                Ok(dbs) => {
                    eprintln!("Search took {:?}", sbh::util::ts() - t);
                    eprintln!("Databases found: {}", dbs.len());
                    for db in dbs.iter() {
                        println!("{}", db.display());
                    }
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1)
                }
            }
        }

        Action::Import { database, files } => match database::import(&database, &files).await {
            Ok(what) => {
                println!("{:?}", what);
            }
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::New { path } => match database::create(&path).await {
            Ok(_) => {
                println!("Database created");
            }
            Err(e) => {
                eprintln!("{:?}", e);
                std::process::exit(1)
            }
        },

        Action::Debug { action } => match action {
            DebugAction::Database { path } => match database::validate(&path).await {
                Ok(_) => {
                    println!("Database valid");
                }
                Err(e) => {
                    eprintln!("{:?}", e);
                    std::process::exit(1)
                }
            }
        }
    }

    Ok(())
}
