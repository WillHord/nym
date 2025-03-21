use crate::{error, helpers, success};
use console::style;

pub fn bulk_toggle_aliases(runcom_file: &str, db_file: &str) {
    let conn = match crate::file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::file_management::database::aliases::get_all_aliases(&conn);

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

pub fn add_alias(rc_file: &str, db_file: &str) {
    let command = match inquire::Text::new("Enter the command:").prompt() {
        Ok(cmd) => {
            if !cmd.is_empty() {
                cmd
            } else {
                error!("Please enter a valid command:");
                return;
            }
        }
        Err(_) => {
            return;
        }
    };
    // TODO:Add validation to command before asking for description (put command ask in loop)

    let description = inquire::Text::new("Enter the description:")
        .prompt()
        .unwrap_or("".to_string());
    // TODO: Allow adding to a group
    crate::commands::aliases::add::add_alias(rc_file, db_file, &command, &description, 1);
}

pub fn bulk_remove_aliases(runcom_file: &str, db_file: &str) {
    let conn = match crate::file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::file_management::database::aliases::get_all_aliases(&conn);

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
        return;
    }

    for alias in selected_aliases {
        crate::commands::aliases::remove::remove_alias(runcom_file, db_file, &alias, true);
    }

    success!("Aliases removed");
}

pub fn rename_alias(runcom_file: &str, db_file: &str) {
    let conn = match crate::file_management::database::setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = crate::file_management::database::aliases::get_all_aliases(&conn);

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
