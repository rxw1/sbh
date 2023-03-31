use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Search for Session Buddy databases and print out
    /// their path
    Search {
        /// Path to search for databases
        #[arg(short, long, value_name = "PATH")]
        path: Option<PathBuf>
    },

    /// Create JSON backups from a Session Buddy database
    Backup {
        /// Database to backup
        #[arg(value_name = "DATABASE")]
        db: PathBuf,

        /// Output <FILENAME>
        #[arg(short, long, value_name = "FILENAME")]
        out: Option<PathBuf>
    },

    /// Import JSON backups to a Session Buddy database
    Import {
        /// Path to the database where the data should be
        /// imported to
        #[arg(short, long)]
        database: PathBuf,

        /// File that should be imported. Can be a JSON file
        /// exported by the Session Buddy
        /// extension or another database
        #[arg(trailing_var_arg = true)]
        files: Vec<PathBuf>
    },

    /// Create a new database
    New {
        #[arg()]
        path: String
    },

    /// Various debug actions
    Debug {
        #[command(subcommand)]
        action: DebugAction
    }
}

#[derive(Subcommand, Debug)]
pub enum DebugAction {
    Database {
        #[arg()]
        path: String
    }
}
