extern crate clap;

mod file_management;

mod install;
mod list;
mod manage;
mod sync;
use crate::list::list_aliases;

use clap::{Arg, ArgAction, Command};

const JSON_FILE: &str = ".aliases.json";
const ALIAS_FILE: &str = ".aliases";

fn main() {
    let matches = Command::new("nym")
        .version("0.1.0")
        .author("Will Hord")
        .about("A simple alias manager")
        .subcommand(
            Command::new("list").about("List all aliases").arg(
                Arg::new("disabled")
                    .long("disabled")
                    .short('d')
                    .help("List disabled aliases")
                    .action(ArgAction::SetTrue),
            ),
        )
        .subcommand(
            Command::new("add")
                .about("Create a new alias")
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
        .subcommand(
            Command::new("rm").about("Remove an alias by name").arg(
                Arg::new("name")
                    .help("The name of the alias to remove")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("toggle").about("Toggle an alias by name").arg(
                Arg::new("name")
                    .help("The name of the alias to toggle")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("sync")
                .about("Sync aliases between json file and alias file")
                .arg(
                    Arg::new("file")
                        .help("The file to sync with")
                        .required(true),
                ),
        )
        // TODO: Impliment these commands
        // .subcommand(
        //     Command::new("install")
        // )
        // .subcommand(
        //     Command::new("uninstall")
        // )
        // .subcommand(
        //     Command::new("man")
        // )
        .get_matches();

    match matches.subcommand() {
        Some(("list", flags)) => {
            println!("Listing  aliases");
            list_aliases(
                JSON_FILE,
                *flags.get_one::<bool>("disabled").unwrap_or(&false),
            );
        }
        Some(("add", sub_m)) => {
            let command = sub_m.get_one::<String>("command").unwrap();
            let description = sub_m.get_one::<String>("description");
            crate::manage::add_alias_command(JSON_FILE, ALIAS_FILE, command, description);
        }
        Some(("rm", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::manage::remove_alias_command(JSON_FILE, ALIAS_FILE, name);
        }
        Some(("toggle", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::manage::toggle_alias_command(JSON_FILE, ALIAS_FILE, name);
        }
        Some(("sync", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
        }
        _ => {
            panic!("Invalid command");
            // println!("TODO: Open up manager")
        }
    }
}
