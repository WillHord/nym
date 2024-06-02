extern crate clap;

use clap::{Command, Arg};

fn main() {
    println!("Starting nym");
    let matches = Command::new("nym")
        .version("0.1.0")
        .author("Will Hord")
        .about("A simple alias manager")
        .subcommand(Command::new("list").about("List all aliases"))
        .subcommand(
            Command::new("add")
                .about("Create a new alias")
                .arg(
                    Arg::new("name")
                        .help("The name of the alias - this is what you will type")
                        .required(true),
                )
                .arg(
                    Arg::new("command")
                        .help("The command to run when the alias is called")
                        .required(true),
                )
                .arg(
                    Arg::new("description")
                        .help("A description of the aliase")
                        .required(false),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        Some(("list", _)) => {
            println!("Listing all aliases")
        }
        Some(("add", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let command = sub_m.get_one::<String>("command").unwrap();
            let description = sub_m.get_one::<String>("description");
            println!("Adding alias: {} -> {}", name, command);
            if let Some(desc) = description {
                println!("Description: {}", desc);
            }
        }
        _ => {
            println!("Open up manager")
        }
    }
}
