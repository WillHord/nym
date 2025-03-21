use crate::file_management::database::aliases::add_alias as db_add_alias;
use crate::file_management::update_runcom;
use crate::{
    error,
    file_management::{
        database::{aliases::get_alias_by_name, setupdb},
        Alias,
    },
    success,
};
use console::style;

use super::validate_alias;

pub fn add_alias(
    runcom_file: &str,
    db_file: &str,
    command: &str,
    description: &str,
    group_id: i32,
) {
    // Split command by = (name=value)
    let alias_command = command.trim_start_matches("alias ");
    let split_command: Vec<&str> = alias_command.split('=').collect();
    let name = split_command[0];
    let value = split_command.last().copied().unwrap();

    // surround value with quotes if not already existing
    let command = if value.starts_with('"') && value.ends_with('"')
        || value.starts_with('\'') && value.ends_with('\'')
    {
        format!("{}={}", name, value)
    } else {
        format!("{}=\"{}\"", name, value)
    };

    let alias_command = match validate_alias(&command) {
        true => command.to_string(),
        false => {
            error!("Command must be in format alias_name=\"command\"");
            return;
        }
    };

    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to alias database");
            return;
        }
    };

    let _ = match get_alias_by_name(&conn, name) {
        Ok(_) => {
            eprintln!("NAME: {}", name);
            error!("Alias already exists");
            return;
        }
        Err(_) => false,
    };

    let mut alias_command = alias_command.split('=').collect::<Vec<&str>>()[1].to_string();
    if alias_command.starts_with('"') {
        alias_command.remove(0);
        if alias_command.ends_with('"') {
            alias_command.pop();
        }
    }

    let alias = Alias {
        name: name.to_string(),
        command: alias_command,
        description: description.to_string(),
        enabled: true,
        group_id,
    };

    match db_add_alias(&conn, &alias) {
        Ok(()) => {
            success!("Alias created successfully");
            println!(
                "Please run {} to activate changes",
                style("`exec \"$SHELL\"`").bold().italic()
            );
        }
        Err(_) => {
            error!("issue creating alias");
        }
    }

    update_runcom(runcom_file, db_file);
}

#[cfg(test)]
mod tests {
    use crate::file_management::{database::aliases::get_all_aliases, runcom::read_aliases};

    use super::*;

    #[test]
    fn test_add_alias_command() {
        let db_path = "test_add_alias_command.db";
        let rc_path = "test_add_alias_command_rc";
        let command1 = r#"alias test1="echo "test command 1"""#;
        let conn = setupdb(db_path).unwrap();

        add_alias(rc_path, db_path, command1, "", 1);

        let aliases = get_all_aliases(&conn);
        let alias_truth = vec![Alias {
            name: "test1".to_string(),
            command: r#"echo "test command 1""#.to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 1,
        }];
        assert_eq!(aliases, alias_truth);

        let rc_aliases = read_aliases(rc_path).unwrap();
        assert_eq!(
            rc_aliases,
            vec![Alias {
                name: "test1".to_string(),
                command: r#"echo "test command 1""#.to_string(),
                description: "".to_string(),
                enabled: true,
                // TODO: This will need to be changed when runcom get_all_aliases can detect group_id
                group_id: 0,
            }]
        );

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
        std::fs::remove_file(rc_path).expect("Error cleaning up test files");
    }
}
