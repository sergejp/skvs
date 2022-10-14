use clap::{parser::ArgMatches, Arg, ArgAction, Command};
use kvs::KvStore;
use std::process;

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
