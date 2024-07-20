mod file_management;
mod helpers;
mod new_file_management;

mod commands;
mod install;
mod list;
mod manager;
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
        .subcommand(Command::new("test"))
        .get_matches();

    // Get json and alias files - if they don't exist throw error (unless subcommand install is called)
    let home_dir = dirs::home_dir().unwrap();
    let json_file_path: String = home_dir
        .join(".nym.json")
        .into_os_string()
        .into_string()
        .unwrap();
    let json_file: &str = json_file_path.as_str();

    let alias_file: String = crate::file_management::json::get_alias_file(json_file);
    if alias_file.is_empty()
        && (matches.subcommand().is_none() || matches.subcommand().unwrap().0 != "install")
    {
        helpers::messages::error!(
            format!(
                "Nym config file not found. Please run {} to create the alias file",
                style("`nym install <shell_profile>`").bold()
            ),
            true
        );
    };
    let alias_file: &str = alias_file.as_str();

    match matches.subcommand() {
        Some(("list", flags)) => {
            println!("Listing  aliases");
            crate::list::list_aliases(
                json_file,
                *flags.get_one::<bool>("disabled").unwrap_or(&false),
            );
        }
        Some(("add", sub_m)) => {
            let command = sub_m.get_one::<String>("command").unwrap();
            let description = sub_m.get_one::<String>("description");
            crate::commands::add_alias_command(json_file, alias_file, command, description);
        }
        Some(("rm", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::commands::remove_alias_command(json_file, alias_file, name);
        }
        Some(("toggle", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            crate::commands::toggle_alias_command(json_file, alias_file, name);
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
        Some(("rename", sub_m)) => {
            let old_name = sub_m.get_one::<String>("old_name").unwrap();
            let new_name = sub_m.get_one::<String>("new_name").unwrap();
            crate::commands::rename_alias(json_file, alias_file, old_name, new_name);
        }
        Some(("test", _)) => {
            // let conn = crate::database::setupdb("test.db").unwrap();
            // crate::database::groups::create_group(&conn, "group1");
            // crate::database::aliases::add_alias(
            //     &conn,
            //     &&NewAlias {
            //         name: "test".to_string(),
            //         command: "echo \"test\"".to_string(),
            //         description: "".to_string(),
            //         enabled: true,
            //         group_id: 1,
            //     },
            // );
            // crate::database::groups::get_groups(&conn);
        }
        _ => {
            crate::manager::alias_manager(json_file, alias_file);
        }
    }
}
