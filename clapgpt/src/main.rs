
use std::process;

use clap::{Command, Arg};

fn main() {

    let matches = Command::new("clapgpt")
    .about("An ai command-line tool")
    .version("0.1")
    .author("Yilkash")
    .subcommand(
        Command::new("clapgpt")
        .arg(
        Arg::new("cmd_arg")
        .required(true)
        )
    )
    .get_matches();

    match  matches.subcommand() {

        Some(("clapgpt", arg_matches)) => {
            if let Some(cmd_argument) = arg_matches.get_one::<String>("cmd_arg"){

            println!("Command line argument {:?}", cmd_argument);
            }

        },
        _ => {

            eprintln!("Invalid input");
            process::exit(1);
        }
    }
    println!("Hello, world!");
}
