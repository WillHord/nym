use crate::file_management::aliases::{append_alias_to_alias_file, remove_alias_from_alias_file};
use crate::file_management::{
    json::{
        add_alias, check_alias_exists, fuzzy_get_alias, remove_alias_by_name, toggle_alias_by_name,
    },
    Alias,
};

pub fn add_alias_command(
    json_file: &str,
    alias_file: &str,
    command: &str,
    description: Option<&String>,
) {
    // Check if command is in fotmat alias_name="command"
    // If not, add quotes around command

    let alias_command = if command.contains('=') {
        command.to_string()
    } else {
        panic!("Command must be in format alias_name=\"command\"");
    };

    let name: &str = alias_command.split('=').collect::<Vec<&str>>()[0];

    // Check if alias already exists
    // If it does, return error
    if check_alias_exists(name, json_file) {
        // TODO: Create better error message (with color)
        panic!("Alias already exists");
    }

    let alias = Alias {
        name: name.to_string(),
        command: alias_command.split('=').collect::<Vec<&str>>()[1].to_string(),
        description: description.unwrap_or(&"".to_string()).to_string(),
        enabled: true,
    };

    add_alias(&alias, json_file);
    append_alias_to_alias_file(&alias, alias_file);

    println!("Alias added successfully");
}

pub fn remove_alias_command(json_file: &str, alias_file: &str, name: &str) {
    // Check if name exists in aliases
    // If does not exist call fuzzy_checker to get nearby names
    // Ask if that is the correct command
    // Ask for confirmation deleting Command

    let alias: Alias = match fuzzy_get_alias(name, json_file) {
        Some(alias) => alias,
        None => panic!("Alias not found"),
    };

    if alias.name != name {
        // Ask for confirmation
        println!("Did you mean {}? (y/n)", alias.name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            panic!("Exiting");
        }
    }

    println!("Are you sure you want to delete {}? (y/n)", alias.name);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "y" {
        panic!("Exiting");
    }

    // Remove alias
    remove_alias_by_name(&alias.name, json_file);
    remove_alias_from_alias_file(&alias.name, alias_file);
    println!("Alias removed successfully");
}

pub fn toggle_alias_command(json_file: &str, alias_file: &str, name: &str) {
    // Toggle alias on or off by name
    let mut alias: Alias = match fuzzy_get_alias(name, json_file) {
        Some(alias) => alias,
        None => panic!("Alias not found"),
    };

    // TODO: Move to new function
    if alias.name != name {
        // Ask for confirmation
        println!("Did you mean {}? (y/n)", alias.name);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            panic!("Exiting");
        }
    }

    toggle_alias_by_name(&alias.name, json_file);
    if alias.enabled {
        alias.enabled = false;
        remove_alias_from_alias_file(&alias.name, alias_file)
    } else {
        alias.enabled = true;
        append_alias_to_alias_file(&alias, alias_file);
    }

    println!(
        "Alias {} is now {}",
        alias.name,
        if alias.enabled { "enabled" } else { "disabled" }
    );
}
