use std::path::PathBuf;

use clap::Parser;
use serde::ser::StdError;

use sbh::session_buddy::backup::Backup;
use sbh::session_buddy::find_databases;
use sbh::session_buddy::get_path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Limit rows
    #[arg(short)]
    limit: Option<i64>,

    /// Backup database to JSON. The Result should be identical with the export produced by the
    /// Session Buddy extension.
    #[arg(short)]
    backup: bool,

    /// Session Buddy databases
    #[arg(trailing_var_arg = true)]
    databases: Vec<String>,
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
        let backup = Backup::new(db, args.limit).await?;
        println!("{}", serde_json::to_string(&backup).unwrap());
    }
    Ok(())
}
