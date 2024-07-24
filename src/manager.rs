use crate::helpers::{
    self,
    messages::{error, success},
};
use console::style;
use inquire::Confirm;

#[derive(Clone)]
enum ManagerOption {
    ListAliases,
    AddAlias,
    RemoveAlias,
    RenameAlias,
    ToggleAlias,
    Quit,
}

impl std::fmt::Display for ManagerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ManagerOption::ListAliases => write!(f, "List aliases"),
            ManagerOption::AddAlias => write!(f, "Add alias"),
            ManagerOption::RemoveAlias => write!(f, "Remove aliases"),
            ManagerOption::RenameAlias => write!(f, "Rename Alias"),
            ManagerOption::ToggleAlias => write!(f, "Toggle aliases"),
            ManagerOption::Quit => write!(f, "Quit"),
        }
    }
}

fn get_manager_option() -> ManagerOption {
    let active_options: Vec<ManagerOption> = vec![
        ManagerOption::ListAliases,
        ManagerOption::AddAlias,
        ManagerOption::RemoveAlias,
        ManagerOption::RenameAlias,
        ManagerOption::ToggleAlias,
        ManagerOption::Quit,
    ];

    let options: Vec<String> = active_options
        .iter()
        .map(|option| option.to_string())
        .collect();

    let selected_option = match inquire::Select::new("Select an option", options).prompt() {
        Ok(option) => option,
        Err(_) => {
            println!("Exiting nym");
            std::process::exit(0);
        }
    };

    let selected_option: &ManagerOption = active_options
        .iter()
        .find(|option| option.to_string() == selected_option)
        .unwrap();

    selected_option.clone()
}

fn bulk_remove_aliases(runcom_file: &str, db_file: &str) {
    let conn = match crate::new_file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::new_file_management::database::aliases::get_all_aliases(&conn);

    if aliases.is_empty() {
        error!("Could not find any aliases to remove");
        return;
    }

    let alias_names: Vec<String> = aliases.iter().map(|alias| alias.name.clone()).collect();

    let selected_aliases: Vec<String> =
        inquire::MultiSelect::new("Select aliases to remove", alias_names)
            .prompt()
            .unwrap();

    if selected_aliases.is_empty() {
        return;
    }

    if !helpers::questions::yesno!("Are you sure you want to delete these aliases?").unwrap() {
        println!("{}", style("Aborting").yellow());
    }

    for alias in selected_aliases {
        crate::commands::aliases::remove::remove_alias(runcom_file, db_file, &alias, true);
    }

    success!("Aliases removed");
}

fn bulk_toggle_aliases(runcom_file: &str, db_file: &str) {
    let conn = match crate::new_file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::new_file_management::database::aliases::get_all_aliases(&conn);

    if aliases.is_empty() {
        error!("Could not find any aliases to toggle");
        return;
    }

    // Alias_names should be a vector of strings of the names of the aliases as well as if they are disabled or not
    let alias_names: Vec<String> = aliases
        .iter()
        .map(|alias| {
            alias.name.clone()
                + " "
                + if alias.enabled {
                    "(enabled)"
                } else {
                    "(disabled)"
                }
        })
        .collect();

    let selected_aliases: Vec<String> =
        inquire::MultiSelect::new("Select aliases to toggle", alias_names)
            .prompt()
            .unwrap();

    if selected_aliases.is_empty() {
        return;
    }

    for alias in selected_aliases {
        // Remove the (enabled) or (disabled) from the alias name to get the actual alias name
        crate::commands::aliases::edit::toggle_alias(
            runcom_file,
            db_file,
            alias.split(' ').next().unwrap(),
        );
    }

    success!("Aliases toggled");
}

fn rename_alias(runcom_file: &str, db_file: &str) {
    let conn = match crate::new_file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::new_file_management::database::aliases::get_all_aliases(&conn);

    if aliases.is_empty() {
        error!("Could not find any aliases to rename");
        return;
    }

    let alias_names: Vec<String> = aliases.iter().map(|alias| alias.name.clone()).collect();

    let selected_alias: String = inquire::Select::new("Select alias to rename", alias_names)
        .prompt()
        .unwrap();

    // TODO: Test this and add validation
    let new_name = inquire::Text::new("Enter the new name:").prompt().unwrap();

    crate::commands::aliases::edit::rename(runcom_file, db_file, &selected_alias, &new_name);
}

pub fn alias_manager(runcom_file: &str, db_file: &str) {
    println!("Alias manager");
    loop {
        let option: ManagerOption = get_manager_option();
        match option {
            ManagerOption::ListAliases => {
                println!("Listing aliases");
                crate::commands::aliases::list::list_aliases(db_file, false);
            }
            ManagerOption::AddAlias => {
                let command = match inquire::Text::new("Enter the command").prompt() {
                    Ok(cmd) => {
                        if !cmd.is_empty() {
                            cmd
                        } else {
                            error!("Please enter a valid command:");
                            continue;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                };
                let description = inquire::Text::new("Enter the description:")
                    .prompt()
                    .unwrap_or("".to_string());
                crate::commands::aliases::add::add_alias(
                    runcom_file,
                    db_file,
                    &command,
                    &description,
                    1,
                );
            }
            ManagerOption::RemoveAlias => {
                bulk_remove_aliases(runcom_file, db_file);
            }
            ManagerOption::RenameAlias => {
                rename_alias(runcom_file, db_file);
            }
            ManagerOption::ToggleAlias => {
                bulk_toggle_aliases(runcom_file, db_file);
            }
            _ => {
                println!("Exiting nym");
                std::process::exit(0)
            }
        }
    }
}
