mod commands;

mod file_management;
mod helpers;
mod new_file_management;

mod install;
mod list;
mod manager;
mod old_commands;
mod sync;

use clap::{Arg, ArgAction, Command};
use console::style;

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
                        .short('d')
                        .long("description")
                        .value_name("DESCRIPTION")
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
            Command::new("rename")
                .about("Rename an alias")
                .arg(
                    Arg::new("old_name")
                        .help("The name of the alias to rename")
                        .required(true),
                )
                .arg(
                    Arg::new("new_name")
                        .help("The new name of the alias")
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
        // .subcommand(
        //     Command::new("sync")
        //         .about("Sync aliases between json file and alias file")
        //         .arg(
        //             Arg::new("force")
        //                 .long("force")
        //                 .short('f')
        //                 .help("Force sync")
        //                 .action(ArgAction::SetTrue),
        //         ),
        // )
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
        .subcommand(Command::new("test"))
        .get_matches();

    // Get config files - if they don't exist throw error (unless subcommand install is called)
    // TODO: check files are present else throw error (unless subcommand install is called)
    let home_dir = dirs::home_dir().unwrap();
    let nym_dir = home_dir.join(".nym/");
    let nym_db = nym_dir
        .join("nym.db")
        .into_os_string()
        .into_string()
        .unwrap();
    let nymrc = nym_dir
        .join("nymrc")
        .into_os_string()
        .into_string()
        .unwrap();

    println!("Database: {}\n Runcom: {}", nym_db, nymrc);

    match matches.subcommand() {
        Some(("list", flags)) => {
            println!("Listing  aliases");
            crate::commands::aliases::list::list_aliases(
                &nym_db,
                *flags.get_one("disabled").unwrap_or(&false),
            );
        }
        Some(("add", sub_m)) => {
            let command = sub_m.get_one::<String>("command").unwrap().to_string();
            let description = sub_m
                .get_one::<String>("description")
                .unwrap_or(&"".to_string())
                .to_string();
            crate::commands::aliases::add::add_alias(&nymrc, &nym_db, &command, &description, 1);
        }
        Some(("rm", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            // TODO: add force flag
            // TODO: allow for "remove" command as well as "rm"
            crate::commands::aliases::remove::remove_alias(&nymrc, &nym_db, name, false);
        }
        Some(("toggle", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::commands::aliases::edit::toggle_alias(&nymrc, &nym_db, name);
        }
        Some(("man", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::commands::aliases::list::alias_manual(&nym_db, name);
        }
        Some(("install", sub_m)) => {
            let shell_profile = sub_m.get_one::<String>("shell_profile").unwrap();
            crate::install::install(shell_profile);
        }
        Some(("uninstall", sub_m)) => {
            let shell_profile = sub_m.get_one::<String>("shell_profile").unwrap();
            // crate::install::uninstall(json_file, shell_profile);
            unimplemented!("temp disabled - need to refactor uninstall")
        }
        Some(("rename", sub_m)) => {
            let old_name = sub_m.get_one::<String>("old_name").unwrap();
            let new_name = sub_m.get_one::<String>("new_name").unwrap();
            crate::commands::aliases::edit::rename(&nymrc, &nym_db, old_name, new_name);
        }
        _ => {
            panic!("Manager is temporarily deactivated");
            // crate::manager::alias_manager(json_file, alias_file);
        }
    }
}
