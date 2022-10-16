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

/*
fn main() {
    let matches = cli().get_matches();
    execute(&matches);
}

fn cli() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand_required(true)
        .subcommand(
              Command::new("set")
                  .about("Insert or update a value by key in the database")
                  .arg(
                      Arg::new("key")
                          .help("Key to insert into the database")
                          .action(ArgAction::Set)
                          .required(true)
                  )
                  .arg(
                      Arg::new("value")
                          .help("Value to insert into the database")
                          .action(ArgAction::Set)
                          .required(true)
                  )
                  .arg_required_else_help(true)
        )
    .subcommand(
        Command::new("get")
        .about("Get value by key from the database")
        .arg(
            Arg::new("key")
                .help("Original key (string) that was used to insert value into the database. \nIt's safe to call if there is no such entry in the database.")
                .action(ArgAction::Set)
                        .required(true)
        )
        .arg_required_else_help(true)
    )
        .subcommand(
            Command::new("rm")
                .about("Remove value by key from the database")
                .arg(
                    Arg::new("key")
                        .help("Original key (string) that was used to insert value into the database. \nIt's safe to call if there is no such entry in the database.")
            .action(ArgAction::Set)
                        .required(true)
                )
                .arg_required_else_help(true)
        )
}

fn execute(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("set", sub_matches)) => {
            println!(
                "kvs set [KEY] [VALUE] was used: KEY={:?} VALUE={:?}",
                sub_matches.get_one::<String>("key"),
                sub_matches.get_one::<String>("value"),
            );
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(("get", sub_matches)) => {
            println!(
                "kvs get [KEY] was used: {:?}",
                sub_matches.get_one::<String>("key")
            );
            eprintln!("unimplemented");
            process::exit(1);
        }
        Some(("rm", sub_matches)) => {
            println!(
                "kvs rm [KEY] was used: {:?}",
                sub_matches.get_one::<String>("key")
            );
            eprintln!("unimplemented");
            process::exit(1);
        }
        _ => unreachable!("Received unsupported command"),
    }
}
*/
