use clap::{Arg, arg, ArgAction, Command};

fn main() {
    let matches = Command::new("kvs")
        .version("0.0.1")
        .author("Sergej P.")
        .about("Networked Key Value Store")
	.subcommand_required(true)
	.subcommand(
	    Command::new("get")
		.about("Get value by key from the database")
		.arg(
		    Arg::new("key")
		        .help("Original key (string) that was used to insert value into the database")
		        .action(ArgAction::Set)
		)
		.arg_required_else_help(true)
	)
        .get_matches();

    match matches.subcommand() {
        Some(("get", sub_matches)) => println!("kvs get [KEY] was used: {:?}", sub_matches.get_one::<String>("key")),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
