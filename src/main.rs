mod commands;
mod file_management;
mod helpers;
mod install;
mod manager;

use clap::{arg, ArgAction, Command};
use console::style;

fn main() {
    let commands = Command::new("nym")
        .version("0.1.4")
        .author("Will Hord")
        .about("A simple alias manager")
        .subcommand(
            Command::new("list")
                .about("List all aliases")
                .aliases(["ls", "l"])
                .subcommand(
                    Command::new("groups")
                        .about("List all groups")
                        .aliases(["group", "g"]),
                )
                // TODO: Add command to list everything in a group
                .subcommand(
                    Command::new("aliases")
                        .about("List all aliases")
                        .aliases(["alias", "a"])
                        .arg(
                            arg!(-d --disabled "List disabled aliases").action(ArgAction::SetTrue),
                        ),
                    // TODO: Add flags for listing by group
                    // TODO: Change disabled to enabled and allow true or false to be passed
                )
                .subcommand(
                    // TODO: Add ability to symlink script
                    // TODO: Add Ability to copy parent dir of script (eg if python script uses venv to run script)
                    Command::new("scripts")
                        .about("List all scripts")
                        .aliases(["script", "s"]),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("Create a new alias or group")
                .subcommand(
                    Command::new("group").about("Create a new group").arg(
                        arg!(<name> "The name of the group")
                            .num_args(1..)
                            .allow_hyphen_values(true),
                    ),
                )
                .subcommand(
                    Command::new("alias")
                        .about("Create a new alias")
                        .arg(
                            arg!(<command> "The command to run when the alias is called")
                                .num_args(1..)
                                .allow_hyphen_values(true),
                        )
                        .arg(arg!(-d --description [DESCRIPTION] "A description of the aliase"))
                        .arg(arg!(-g --group [GROUP] "The group to add the alias to")),
                )
                .subcommand(
                    Command::new("script")
                        .about("Add a new script")
                        .arg(arg!(<path> "The path to the script"))
                        .arg(arg!(-d --description [DESCRIPTION] "A description of the script"))
                        .arg(arg!(-g --group [GROUP] "The group to add the script to")), // .arg(arg!(-ln --link [LINK] "Use symlink instead of copying script")),
                ),
        )
        .subcommand(
            // TODO: Allow removing multiple at a time (only if it is not a mix of groups and other items)
            Command::new("remove")
                .about("Remove an alias or group by name")
                .aliases(["rm"])
                .arg(arg!(<name> "The name of the item to remove"))
                .arg(arg!(-f --force "Force remove item")),
        )
        .subcommand(
            Command::new("rename")
                .about("Rename an alias or group")
                .arg(arg!(<old_name> "Name of item to rename"))
                .arg(arg!(<new_name> "New Name of item")),
        )
        .subcommand(
            Command::new("toggle")
                .about("Toggle an alias by name")
                .arg(arg!(<name> "The name of the item to toggle")),
        )
        .subcommand(Command::new("install").about("Install Nym").arg(
            arg!(<shell_profile> "The shell profile file to install Nym. E.g. .bashrc, .zshrc"),
        ))
        .subcommand(Command::new("uninstall").about("Uninstall Nym").arg(
            arg!(<shell_profile> "The shell profile file to uninstall Nym. E.g. .bashrc, .zshrc"),
        ))
        .subcommand(
            Command::new("man")
                .about("Open up description of alias")
                .arg(arg!(<name> "The name of the alias to view description of")),
        )
        .subcommand(
            Command::new("update")
                .about("Update alias or script")
                .arg(arg!(<name> "name of item to update"))
                .arg(arg!(-n --new <NEW_ITEM> "Either the new command for the alias or path to updated script"))
            ,
        )
        .subcommand(
            // TODO: Allow creating a new group while moving "move -n group_name"
            Command::new("move")
                .about("Move alias or script to a different group")
                .aliases(["mv"])
                .arg(arg!(<name> "The name of the item to toggle"))
                .arg(arg!(<group> "The name of the group to move the item to").required(false))
                .arg(arg!(-n --new_group <NEW_GROUP> "Create a new group to add alias or script to")),
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
            // TODO: Allow listing specifc group(s) and just aliases or scripts in the group
            Some(("groups", _)) => {
                crate::commands::groups::list::list_groups(&nym_db);
            }
            Some(("aliases", sub_m)) => {
                crate::commands::aliases::list::list_aliases(
                    &nym_db,
                    *sub_m.get_one("disabled").unwrap_or(&false),
                );
            }
            Some(("scripts", _)) => {
                crate::commands::scripts::list::list_scripts(&nym_db);
            }
            _ => {
                crate::commands::groups::list::list_all(&nym_db);
            }
        },
        Some(("add", sub_m)) => {
            match sub_m.subcommand() {
                Some(("group", sub_m)) => {
                    let name: Vec<String> = sub_m
                        .get_many::<String>("name")
                        .unwrap()
                        .map(|s| s.to_string())
                        .collect();
                    crate::commands::groups::add::add_group(&nym_db, &name.join(" "));
                }
                Some(("alias", sub_m)) => {
                    let command_vector: Vec<String> = sub_m
                        .get_many::<String>("command")
                        .unwrap()
                        .map(|s| s.to_string())
                        .collect();

                    println!("command vector: {:?}", command_vector);

                    let command = command_vector.join(" ");

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
            let name = sub_m.get_one::<String>("name").unwrap();
            let force = sub_m.get_one::<bool>("force").unwrap_or(&false);

            match crate::commands::get_item(&nym_db, name, true) {
                Some(crate::commands::Item::Alias(alias)) => {
                    crate::commands::aliases::remove::remove_alias(
                        &nymrc,
                        &nym_db,
                        &alias.name,
                        *force,
                    );
                }
                Some(crate::commands::Item::Group(group)) => {
                    crate::commands::groups::remove::remove_group(
                        &nymrc,
                        &nym_db,
                        &group.name,
                        *force,
                    );
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::remove::remove_script(
                        &nymrc,
                        &nym_db,
                        &script.name,
                        *force,
                    );
                }
                None => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            };
        }
        Some(("toggle", sub_m)) => {
            // TODO: Add ability to specify type
            let name = sub_m.get_one::<String>("name").unwrap();

            match crate::commands::get_item(&nym_db, name, true) {
                Some(crate::commands::Item::Alias(alias)) => {
                    crate::commands::aliases::edit::toggle_alias(&nymrc, &nym_db, &alias.name)
                }
                Some(crate::commands::Item::Group(group)) => {
                    crate::commands::groups::toggle::toggle_group(&nymrc, &nym_db, &group.name)
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::edit::toggle_script(&nymrc, &nym_db, &script.name)
                }
                None => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            };
        }
        Some(("man", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            match crate::commands::get_item(&nym_db, name, false) {
                Some(crate::commands::Item::Alias(alias)) => {
                    crate::commands::aliases::list::alias_manual(&nym_db, &alias.name);
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::list::script_manual(&nym_db, &script.name);
                }
                _ => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            }
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
            let old_name = sub_m.get_one::<String>("old_name").unwrap();
            let new_name = sub_m.get_one::<String>("new_name").unwrap();

            match crate::commands::get_item(&nym_db, old_name, true) {
                Some(crate::commands::Item::Alias(alias)) => {
                    crate::commands::aliases::edit::rename(&nymrc, &nym_db, &alias.name, new_name);
                }
                Some(crate::commands::Item::Group(group)) => {
                    crate::commands::groups::rename::rename_group(
                        &nymrc,
                        &nym_db,
                        &group.name,
                        new_name,
                    );
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::edit::rename_script(
                        &nymrc,
                        &nym_db,
                        &script.name,
                        new_name,
                    );
                }
                _ => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            };
        }
        Some(("move", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let group = sub_m.get_one::<String>("group").unwrap();

            let new_group = sub_m
                .get_one::<String>("new_group")
                .unwrap_or(&"".to_string())
                .to_string();

            match crate::commands::get_item(&nym_db, name, false) {
                Some(crate::commands::Item::Alias(alias)) => {
                    crate::commands::aliases::edit::move_alias_group(
                        &nymrc,
                        &nym_db,
                        &alias.name,
                        group,
                        if new_group.is_empty() {
                            None 
                        } else {
                           Some(&new_group) 
                        }
                    );
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::edit::move_script(
                        &nymrc,
                        &nym_db,
                        &script.name,
                        group,
                    );
                }
                _ => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            }
        }
        Some(("update", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let updated_item = sub_m
                .get_one::<String>("new")
                .unwrap_or(&"".to_string())
                .to_string();

            match crate::commands::get_item(&nym_db, name, false) {
                Some(crate::commands::Item::Alias(alias)) => {
                    println!("Found alias {}", alias.name);
                }
                Some(crate::commands::Item::Script(script)) => {
                    crate::commands::scripts::update::update_script(
                        &nym_db,
                        &nymrc,
                        &script.name,
                        if updated_item.is_empty() {
                            None
                        } else {
                            Some(&updated_item)
                        },
                    );
                    println!("Found script {}", script.name);
                }
                _ => {
                    error!(format!(
                        "Item not found. Try using {} to find the correct item",
                        style("`nym list`").bold()
                    ))
                }
            }
        }
        _ => {
            crate::manager::start_manager(&nymrc, &nym_db);
        }
    }
}
