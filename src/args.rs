use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Search for databases and print out their path
    Search {
        /// Path to search for databases. If no path argument is
        /// given we're trying to figure it out.
        #[arg()]
        path: Option<PathBuf>
    },

    /// Create JSON backups from a Session Buddy database
    Backup {
        /// Search for databases
        //#[arg(conflicts_with = "path")]
        //search: bool,

        /// Output <FILENAME>
        #[arg(short, long, value_name = "FILENAME")]
        out: Option<PathBuf>,

        /// Database to backup
        #[arg(value_name = "DATABASE")]
        path: PathBuf,

    },

    /// Import JSON backups to a Session Buddy database
    Import {
        /// Path to the database where the data should be imported to
        #[arg(short, long)]
        path: PathBuf,

        /// File that should be imported. Can be a JSON file exported
        /// by the Session Buddy extension or another database
        #[arg(trailing_var_arg = true)]
        files: Vec<PathBuf>
    },

    /// Create a new database
    New {
        /// Path to database
        #[arg()]
        path: PathBuf
    },

    /// Print some database statistics
    Stats {
        /// Path to database
        #[arg()]
        path: PathBuf
    },

    /// Print all URLs of a database to stdout
    Dump {
        /// Path to database
        #[arg()]
        path: PathBuf
    },


    /// Print the id of a database
    Id {
        /// Path to database
        #[arg()]
        path: PathBuf
    },

    /// Validate a database or JSON backup
    Validate {
        #[command(subcommand)]
        action: ValidateAction
    },
}

#[derive(Subcommand, Debug)]
pub enum ValidateAction {
    Database {
        /// Path to database
        #[arg()]
        path: PathBuf
    },

    Backup {
        /// Path to database
        #[arg()]
        path: PathBuf
    }
}
