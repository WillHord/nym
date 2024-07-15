use crate::file_management::aliases::{append_alias_to_alias_file, remove_alias_from_alias_file};
use crate::file_management::{
    json::{
        add_alias, check_alias_exists, fuzzy_get_alias, remove_alias_by_name, toggle_alias_by_name,
    },
    Alias,
};
use crate::{error, success};

use console::style;
use dialoguer::Confirm;
use fancy_regex::Regex;

fn confirm_alias(alias: &Alias) -> bool {
    // Ask for confirmation
    let confirm = Confirm::new()
        .with_prompt(format!("Did you mean {}?", alias.name))
        .interact()
        .unwrap();
    confirm
}

fn validate_alias(alias: &str) -> bool {
    let pattern = r#"(?:alias\s+)?(\w+)=([\'"])((?:\\.|(?!\2).)*)\2"#;
    let re = Regex::new(pattern).unwrap();

    match re.is_match(alias) {
        Ok(value) => value,
        Err(_) => {
            error!("Error validating alias");
            false
        }
    }
}

pub fn add_alias_command(
    json_file: &str,
    alias_file: &str,
    command: &str,
    description: Option<&String>,
) {
    // Check if command is in fotmat alias_name="command"
    // If not, add quotes around command

    let alias_command = if validate_alias(command) {
        command.to_string()
    } else {
        error!("Command must be in format alias_name=\"command\"");
        return;
    };

    // If alias starts with alias remove starting alias substring
    let alias_command = alias_command.trim_start_matches("alias ");
    let name: &str = alias_command.split('=').collect::<Vec<&str>>()[0];

    // Check if alias already exists
    // If it does, return error
    if check_alias_exists(name, json_file) {
        error!("Alias Already exists", true);
    }

    let alias = Alias {
        name: name.to_string(),
        command: alias_command.split('=').collect::<Vec<&str>>()[1].to_string(),
        description: description.unwrap_or(&"".to_string()).to_string(),
        enabled: true,
    };

    add_alias(&alias, json_file);
    append_alias_to_alias_file(&alias, alias_file);

    success!("Alias created successfully");
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
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

    if alias.name != name && !confirm_alias(&alias) {
        error!("Please Try again with a different alias", true);
    }

    if !Confirm::new()
        .with_prompt(format!("Are you sure you want to delete {}?", alias.name))
        .interact()
        .unwrap()
    {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    // Remove alias
    remove_alias_by_name(&alias.name, json_file);
    remove_alias_from_alias_file(&alias.name, alias_file);
    success!("Alias removed successfully");
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
}

pub fn toggle_alias_command(json_file: &str, alias_file: &str, name: &str) {
    // Toggle alias on or off by name
    let mut alias: Alias = match fuzzy_get_alias(name, json_file) {
        Some(alias) => alias,
        None => panic!("Alias not found"),
    };

    if alias.name != name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
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
        style(alias.name).italic().bold(),
        if alias.enabled {
            style("enabled").bold().green()
        } else {
            style("disabled").bold().red()
        }
    );
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
}

pub fn rename_alias(json_file: &str, alias_file: &str, old_name: &str, new_name: &str) {
    // Rename alias by name
    let mut alias: Alias = match fuzzy_get_alias(old_name, json_file) {
        Some(alias) => alias,
        None => panic!("Alias not found"),
    };

    if alias.name != old_name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
    }

    remove_alias_by_name(&alias.name, json_file);
    remove_alias_from_alias_file(&alias.name, alias_file);
    alias.name = new_name.to_string();
    append_alias_to_alias_file(&alias, alias_file);
    add_alias(&alias, json_file);

    success!(format!(
        "Alias {} has been renamed to {}",
        style(old_name).bold().italic(),
        style(new_name).italic().bold()
    ));
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_alias() {
        let valid_alias_strings = vec![
            r#"alias alias_name="echo 'test'""#,
            r#"alias_name="echo 'test'""#,
            r#"alias alias_name='echo "test"'"#,
            r#"alias_name='echo "test"'"#,
            r#"alias alias_name="echo \"nested 'test'\"""#,
            r#"alias_name='echo \'nested "test"\'""#,
            r#"alias alias_name="echo \\"test\\"""#,
            r#"alias_name="echo \\"test\\"""#,
        ];

        let invalid_alias_strings = vec![
            r#"alias alias_name = "echo 'test'"#,
            r#"alias alias_name= "echo 'test'"#,
            r#"alias_name= "echo 'test'"#,
            r#"alias alias_name="#,
            r#"alias alias name="echo 'test'"#,
            r#"alias echo 'test'"#,
            r#"alias test="echo 'test'"#,
        ];

        for alias in valid_alias_strings {
            assert!(validate_alias(alias));
        }

        for alias in invalid_alias_strings {
            assert!(!validate_alias(alias));
        }
    }
}
