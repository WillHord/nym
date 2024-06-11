extern crate clap;

mod file_management;

mod install;
mod list;
mod manage;
mod sync;
use crate::list::list_aliases;

use clap::{Arg, ArgAction, Command};
use console::style;

// const JSON_FILE: &str = ".aliases.json"; // This should be in the home directory
// const JSON_FILE: &str = dirs::home_dir()
//     .unwrap()
//     .join(".aliases.json")
//     .to_str()
//     .unwrap();
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
                    Arg::new("force")
                        .long("force")
                        .short('f')
                        .help("Force sync")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("install").about("Install Nym").arg(
                Arg::new("shell_profile")
                    .help("The shell profile file to install Nym. E.g. .bashrc, .zshrc")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("uninstall").about("Uninstall Nym").arg(
                Arg::new("shell_profile")
                    .help("The shell profile file to uninstall Nym. E.g. .bashrc, .zshrc")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("man")
                .about("Open up description of alias")
                .arg(
                    Arg::new("name")
                        .required(true)
                        .help("The name of the alias to view description of"),
                ),
        )
        .get_matches();

    // Get json and alias files - if they don't exist throw error (unless subcommand install is called)
    let home_dir = dirs::home_dir().unwrap();
    let json_file_path: String = home_dir
        .join(".aliases.json")
        .into_os_string()
        .into_string()
        .unwrap();
    let json_file: &str = json_file_path.as_str();

    let alias_file: String = crate::file_management::json::get_alias_file(json_file);
    if alias_file.is_empty() && matches.subcommand().unwrap().0 != "install" {
        eprintln!(
            "{}: Alias file not found. Please run {} to create the alias file",
            style("Error").red().bold(),
            style("`nym install`").bold()
        );
        std::process::exit(1);
    };
    let alias_file: &str = alias_file.as_str();

    match matches.subcommand() {
        Some(("list", flags)) => {
            println!("Listing  aliases");
            list_aliases(
                json_file,
                *flags.get_one::<bool>("disabled").unwrap_or(&false),
            );
        }
        Some(("add", sub_m)) => {
            let command = sub_m.get_one::<String>("command").unwrap();
            let description = sub_m.get_one::<String>("description");
            crate::manage::add_alias_command(json_file, alias_file, command, description);
        }
        Some(("rm", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::manage::remove_alias_command(json_file, alias_file, name);
        }
        Some(("toggle", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::manage::toggle_alias_command(json_file, alias_file, name);
        }
        Some(("sync", sub_m)) => {
            let force: bool = *sub_m.get_one::<bool>("force").unwrap_or(&false);
            crate::sync::sync_aliases(json_file, alias_file, force);
        }
        Some(("man", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::list::alias_manual(json_file, name);
        }
        Some(("install", sub_m)) => {
            let shell_profile = sub_m.get_one::<String>("shell_profile").unwrap();
            crate::install::install(json_file, shell_profile);
        }
        Some(("uninstall", sub_m)) => {
            let shell_profile = sub_m.get_one::<String>("shell_profile").unwrap();
            crate::install::uninstall(json_file, shell_profile);
        }
        _ => {
            eprintln!(
                "{} No command provided. Please run {} for help",
                style("Error:").red().bold(),
                style("`nym --help`").bold()
            );
        }
    }
}
