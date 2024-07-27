mod commands;
mod file_management;
mod helpers;
mod install;
mod manager;

use clap::{Arg, ArgAction, Command};
use console::style;

fn main() {
    let commands = Command::new("nym")
        .version("0.1.2")
        .author("Will Hord")
        .about("A simple alias manager")
        .subcommand(
            Command::new("list")
                .about("List all aliases")
                .subcommand(Command::new("group").about("List all groups"))
                .subcommand(
                    Command::new("alias").about("List all aliases").arg(
                        Arg::new("disabled")
                            .long("disabled")
                            .short('d')
                            .help("List disabled aliases")
                            .action(ArgAction::SetTrue),
                    ),
                    // TODO: Add flags for listing by group
                    // TODO: Change disabled to enabled and allow true or false to be passed
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Create a new alias or group")
                .subcommand(
                    Command::new("group").about("Create a new group").arg(
                        Arg::new("name")
                            .help("The name of the group")
                            .required(true),
                    ),
                )
                .subcommand(
                    Command::new("alias")
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
                        )
                        .arg(
                            Arg::new("group")
                                .short('g')
                                .long("group")
                                .value_name("GROUP")
                                .help("The group to add the alias to")
                                .required(false),
                        ),
                ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove an alias or group by name")
                .subcommand(
                    Command::new("group")
                        .about("Remove a group by name")
                        .arg(
                            Arg::new("name")
                                .help("The name of the group to remove")
                                .required(true),
                        )
                        .arg(
                            Arg::new("force")
                                .help("Force remove group")
                                .short('f')
                                .long("force")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        ),
                )
                .subcommand(
                    Command::new("alias")
                        .about("Remove an alias by name")
                        .arg(
                            Arg::new("name")
                                .help("The name of the alias to remove")
                                .required(true),
                        )
                        .arg(
                            Arg::new("force")
                                .help("Force remove alias")
                                .short('f')
                                .long("force")
                                .action(ArgAction::SetTrue)
                                .required(false),
                        ),
                ),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename an alias or group")
                .subcommand(
                    Command::new("alias")
                        .about("Rename an alias by name")
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
                    Command::new("group")
                        .about("Rename a group by name")
                        .arg(
                            Arg::new("old_name")
                                .help("The name of the group to rename")
                                .required(true),
                        )
                        .arg(
                            Arg::new("new_name")
                                .help("The new name of the group")
                                .required(true),
                        ),
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
        );
    let matches = commands.clone().get_matches();

    // Get config files - if they don't exist throw error (unless subcommand install is called)
    let home_dir = dirs::home_dir().unwrap();
    let nym_dir = home_dir.join(".nym/");

    if !nym_dir.exists()
        && (matches.subcommand().is_none() || matches.subcommand().unwrap().0 != "install")
    {
        eprintln!(
            "{}: Nym config dir not found. Please run {} to create dir",
            style("Error").red().bold(),
            style("`nym install <shell_profile>`").bold()
        );
        std::process::exit(1);
    }

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

    match matches.subcommand() {
        Some(("list", sub_m)) => {
            match sub_m.subcommand() {
                Some(("group", _)) => {
                    println!("Listing groups");
                    crate::commands::groups::list::list_groups(&nym_db);
                }
                Some(("alias", sub_m)) => {
                    println!("Listing aliases");
                    crate::commands::aliases::list::list_aliases(
                        &nym_db,
                        *sub_m.get_one("disabled").unwrap_or(&false),
                    );
                }
                _ => {
                    // TODO: Change this to list aliases by group
                    // Display help message
                    commands
                        .find_subcommand("list")
                        .unwrap()
                        .clone()
                        .print_help()
                        .unwrap();
                }
            }
        }
        Some(("add", sub_m)) => {
            match sub_m.subcommand() {
                Some(("group", sub_m)) => {
                    println!("Adding group");
                    let name = sub_m.get_one::<String>("name").unwrap();
                    crate::commands::groups::add::add_group(&nym_db, name);
                }
                Some(("alias", sub_m)) => {
                    println!("Adding alias");
                    let command = sub_m.get_one::<String>("command").unwrap().to_string();
                    let description = sub_m
                        .get_one::<String>("description")
                        .unwrap_or(&"".to_string())
                        .to_string();
                    let group_name = sub_m
                        .get_one::<String>("group")
                        .unwrap_or(&"".to_string())
                        .to_string();

                    let group_id = if group_name.is_empty() {
                        1
                    } else {
                        crate::commands::groups::ask_fuzzy_get(&nym_db, &group_name)
                            .unwrap()
                            .id
                    };

                    crate::commands::aliases::add::add_alias(
                        &nymrc,
                        &nym_db,
                        &command,
                        &description,
                        group_id,
                    );
                }
                _ => {
                    // Display help message
                    commands
                        .find_subcommand("add")
                        .unwrap()
                        .clone()
                        .print_help()
                        .unwrap();
                }
            }
        }
        Some(("remove", sub_m)) => {
            match sub_m.subcommand() {
                Some(("group", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    let force = sub_m.get_one::<bool>("force").unwrap_or(&false);
                    crate::commands::groups::remove::remove_group(&nymrc, &nym_db, name, *force);
                }
                Some(("alias", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    let force = sub_m.get_one::<bool>("force").unwrap_or(&false);
                    crate::commands::aliases::remove::remove_alias(&nymrc, &nym_db, name, *force);
                }
                _ => {
                    // Display help message
                    commands
                        .find_subcommand("remove")
                        .unwrap()
                        .clone()
                        .print_help()
                        .unwrap();
                }
            }
        }
        Some(("toggle", sub_m)) => {
            // TODO: Add support for toggling groups, and scripts (when added)
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
            crate::install::uninstall(shell_profile);
        }
        Some(("rename", sub_m)) => {
            match sub_m.subcommand() {
                Some(("group", sub_m)) => {
                    let old_name = sub_m.get_one::<String>("old_name").unwrap();
                    let new_name = sub_m.get_one::<String>("new_name").unwrap();
                    crate::commands::groups::rename::rename_group(
                        &nymrc, &nym_db, old_name, new_name,
                    );
                }
                Some(("alias", sub_m)) => {
                    let old_name = sub_m.get_one::<String>("old_name").unwrap();
                    let new_name = sub_m.get_one::<String>("new_name").unwrap();
                    crate::commands::aliases::edit::rename(&nymrc, &nym_db, old_name, new_name);
                }
                _ => {
                    // Display help message
                    commands
                        .find_subcommand("rename")
                        .unwrap()
                        .clone()
                        .print_help()
                        .unwrap();
                }
            }
        }
        _ => {
            crate::manager::alias_manager(&nymrc, &nym_db);
        }
    }
}
