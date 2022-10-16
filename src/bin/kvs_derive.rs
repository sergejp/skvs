use clap::{Parser, Subcommand};
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Inserts or updates a value by key in the database
    Set {
        /// Key to insert into the database
        key: String,
        /// Value to insert into the database
        value: String,
    },
    /// Gets value by key from the database
    Get {
        /// Original key (string) that was used to insert value into the database. It's safe to call if there is no such entry in the database.
        key: String,
    },
    /// Removes value by key from the database
    Rm {
        /// Original key (string) that was used to insert value into the database. It's safe to call if there is no such entry in the database.
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        Subcommands::Set { key, value } => {
            println!(
                "kvs set [KEY] [VALUE] was used: KEY={:?} VALUE={:?}",
                key, value
            );
            eprintln!("unimplemented");
            process::exit(1);
        }
        Subcommands::Get { key } => {
            println!("kvs get [KEY] was used: KEY={:?}", key);
            eprintln!("unimplemented");
            process::exit(1);
        }
        Subcommands::Rm { key } => {
            println!("kvs rm [KEY] was used: KEY={:?}", key);
            eprintln!("unimplemented");
            process::exit(1);
        }
    }
}
