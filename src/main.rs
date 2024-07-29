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
                .subcommand(Command::new("groups").about("List all groups"))
                // TODO: Add command to list everything in a group
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
                )
                .subcommand(Command::new("scripts").about("List all scripts")),
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
                )
                .subcommand(
                    Command::new("script")
                        .about("Add a new script")
                        .arg(
                            Arg::new("path")
                                .help("The path to the script")
                                .required(true),
                        )
                        .arg(
                            Arg::new("description")
                                .short('d')
                                .long("description")
                                .value_name("DESCRIPTION")
                                .help("A description of the script")
                                .required(false),
                        )
                        .arg(
                            Arg::new("group")
                                .short('g')
                                .long("group")
                                .value_name("GROUP")
                                .help("The group to add the script to")
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
                )
                .subcommand(
                    Command::new("script")
                        .about("Remove a script by name")
                        .arg(
                            Arg::new("name")
                                .help("The name of the script to remove")
                                .required(true),
                        )
                        .arg(
                            Arg::new("force")
                                .help("Force remove script")
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
                )
                .subcommand(
                    Command::new("script")
                        .about("Rename a script by name")
                        .arg(
                            Arg::new("old_name")
                                .help("The name of the script to rename")
                                .required(true),
                        )
                        .arg(
                            Arg::new("new_name")
                                .help("The new name of the script")
                                .required(true),
                        ),
                ),
        )
        .subcommand(
            Command::new("toggle")
                .about("Toggle an alias by name")
                .subcommand(
                    Command::new("alias").about("Toggle an alias by name").arg(
                        Arg::new("name")
                            .help("The name of the alias to toggle")
                            .required(true),
                    ),
                )
                // .subcommand(
                //     Command::new("group").about("Toggle a group by name").arg(
                //         Arg::new("name")
                //             .help("The name of the group to toggle")
                //             .required(true),
                //     ),
                // )
                .subcommand(
                    Command::new("script").about("Toggle a script by name").arg(
                        Arg::new("name")
                            .help("The name of the script to toggle")
                            .required(true),
                    ),
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
        .subcommand(
            Command::new("move")
                .about("Move alias or script to a different group")
                .subcommand(
                    Command::new("alias")
                        .about("Move alias to a different group")
                        .arg(
                            Arg::new("alias")
                                .required(true)
                                .help("The name of the alias to move"),
                        )
                        .arg(
                            Arg::new("group")
                                .help("The name of the group to move the alias to")
                                .required(true),
                        ),
                )
                .subcommand(
                    Command::new("script")
                        .about("Move script to a different group")
                        .arg(
                            Arg::new("script")
                                .required(true)
                                .help("The name of the script to move"),
                        )
                        .arg(
                            Arg::new("group")
                                .help("The name of the group to move the script to")
                                .required(true),
                        ),
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
        Some(("list", sub_m)) => match sub_m.subcommand() {
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
            Some(("scripts", _)) => {
                println!("Listing scripts");
                crate::commands::scripts::list::list_scripts(&nym_db);
            }
            _ => {
                crate::commands::groups::list::list_all(&nym_db);
            }
        },
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
                Some(("script", sub_m)) => {
                    println!("Adding script");
                    let path = sub_m.get_one::<String>("path").unwrap().to_string();
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

                    crate::commands::scripts::add::add_script(
                        &nymrc,
                        &nym_db,
                        &path,
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
                Some(("script", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    let force = sub_m.get_one::<bool>("force").unwrap_or(&false);
                    crate::commands::scripts::remove::remove_script(&nymrc, &nym_db, name, *force);
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
            // TODO: Auto detect type of toggle and make group toggle a new command
            match sub_m.subcommand() {
                Some(("alias", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    crate::commands::aliases::edit::toggle_alias(&nymrc, &nym_db, name);
                }
                // Some(("group", sub_m)) => {
                //     // let name = sub_m.get_one::<String>("name").unwrap();
                //     todo!("Yep I didn't finish this yet");
                //     // crate::commands::groups::toggle::toggle_group(&nymrc, &nym_db, name);
                // }
                Some(("script", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    crate::commands::scripts::edit::toggle_script(&nymrc, &nym_db, name);
                }
                _ => {
                    // Display help message
                    commands
                        .find_subcommand("toggle")
                        .unwrap()
                        .clone()
                        .print_help()
                        .unwrap();
                }
            }
        }
        Some(("man", sub_m)) => {
            // TODO: Auto detect type item
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
                Some(("script", sub_m)) => {
                    let old_name = sub_m.get_one::<String>("old_name").unwrap();
                    let new_name = sub_m.get_one::<String>("new_name").unwrap();
                    crate::commands::scripts::edit::rename_script(
                        &nymrc, &nym_db, old_name, new_name,
                    );
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
        Some(("move", sub_m)) => {
            match sub_m.subcommand() {
                Some(("alias", sub_m)) => {
                    let alias = sub_m.get_one::<String>("alias").unwrap();
                    let group = sub_m.get_one::<String>("group").unwrap();
                    crate::commands::aliases::edit::move_alias_group(&nymrc, &nym_db, alias, group);
                }
                Some(("script", sub_m)) => {
                    let script = sub_m.get_one::<String>("script").unwrap();
                    let group = sub_m.get_one::<String>("group").unwrap();
                    crate::commands::scripts::edit::move_script(&nymrc, &nym_db, script, group);
                }
                _ => {
                    // Display help message
                    commands
                        .find_subcommand("move")
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
