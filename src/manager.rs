use crate::helpers::messages::{error, success};
use console::style;

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

    // let confirm = inquire::Confirm::new("Are you sure?").prompt().unwrap();

    selected_option.clone()
}

fn bulk_remove_aliases(json_file: &str, alias_file: &str) {
    let aliases = crate::file_management::json::get_aliases_from_file(json_file);

    if aliases.aliases.is_empty() {
        println!(
            "{}: {}",
            style("Error").red().bold(),
            style("Could not find any aliases to remove")
        );
        return;
    }

    let alias_names: Vec<String> = aliases
        .aliases
        .iter()
        .map(|alias| alias.name.clone())
        .collect();

    let selected_aliases: Vec<String> =
        inquire::MultiSelect::new("Select aliases to remove", alias_names)
            .prompt()
            .unwrap();

    if selected_aliases.is_empty() {
        return;
    }

    if !inquire::Confirm::new("Are you sure you want to delete these aliases?")
        .prompt()
        .unwrap()
    {
        println!("{}", style("Aborting").yellow());
    }

    for alias in selected_aliases {
        crate::file_management::json::remove_alias_by_name(&alias, json_file);
        crate::file_management::aliases::remove_alias_from_alias_file(&alias, alias_file);
    }

    println!("{} Aliases removed", style("Success:").green().bold());
}

fn bulk_toggle_aliases(json_file: &str, alias_file: &str) {
    let aliases = crate::file_management::json::get_aliases_from_file(json_file);

    if aliases.aliases.is_empty() {
        println!(
            "{}: {}",
            style("Error").red().bold(),
            style("Could not find any aliases to toggle")
        );
        return;
    }

    // Alias_names should be a vector of strings of the names of the aliases as well as if they are disabled or not
    let alias_names: Vec<String> = aliases
        .aliases
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
        crate::commands::toggle_alias_command(
            json_file,
            alias_file,
            alias.split(' ').next().unwrap(),
        );
    }

    println!("{} Aliases toggled", style("Success:").green().bold());
    // println!(
    //     "Please run {} to apply the changes",
    //     style("`exec $SHELL`").bold()
    // );
}

fn rename_alias(json_file: &str, alias_file: &str) {
    let aliases = crate::file_management::json::get_aliases_from_file(json_file);

    if aliases.aliases.is_empty() {
        println!(
            "{}: {}",
            style("Error").red().bold(),
            style("Could not find any aliases to rename")
        );
        return;
    }

    let alias_names: Vec<String> = aliases
        .aliases
        .iter()
        .map(|alias| alias.name.clone())
        .collect();

    let selected_alias: String = inquire::Select::new("Select alias to rename", alias_names)
        .prompt()
        .unwrap();

    let new_name = inquire::Text::new("Enter the new name").prompt().unwrap();

    crate::commands::rename_alias(json_file, alias_file, &selected_alias, &new_name);
}

pub fn alias_manager(json_file: &str, alias_file: &str) {
    println!("Alias manager");
    loop {
        let option: ManagerOption = get_manager_option();
        match option {
            ManagerOption::ListAliases => {
                println!("Listing aliases");
                crate::list::list_aliases(json_file, false);
            }
            ManagerOption::AddAlias => {
                let command = match inquire::Text::new("Enter the command").prompt() {
                    Ok(cmd) => {
                        if !cmd.is_empty() {
                            cmd
                        } else {
                            println!(
                                "{}: {}",
                                style("Error").red().bold(),
                                style("Please enter a valid command")
                            );
                            continue;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                };
                let description = inquire::Text::new("Enter the description")
                    .prompt()
                    .unwrap();
                crate::commands::add_alias_command(
                    json_file,
                    alias_file,
                    &command,
                    Some(&description),
                );
            }
            ManagerOption::RemoveAlias => {
                bulk_remove_aliases(json_file, alias_file);
            }
            ManagerOption::RenameAlias => {
                rename_alias(json_file, alias_file);
            }
            ManagerOption::ToggleAlias => {
                bulk_toggle_aliases(json_file, alias_file);
            }
            _ => {
                println!("Exiting nym");
                std::process::exit(0)
            }
        }
    }
}
