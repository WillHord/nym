use crate::{
    commands::{
        aliases::{confirm_alias, fuzzy_get_alias},
        fuzzy_get_group,
    },
    error,
    file_management::{
        database::{aliases::update_alias, setupdb},
        update_runcom,
    },
    success,
};
use console::style;
use inquire::Confirm;

// TODO: abstract out connection and fuzzy get alias to private function

pub fn rename(runcom_file: &str, db_file: &str, old_name: &str, new_name: &str) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let mut alias = match fuzzy_get_alias(old_name, db_file) {
        Some(alias) => alias,
        None => {
            error!("Alias not found");
            return;
        }
    };

    if alias.name != old_name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
    }

    let old_name = alias.name;
    alias.name = new_name.to_string();

    let _ = match update_alias(&conn, &old_name, alias.clone()) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not rename alias");
            return;
        }
    };

    update_runcom(runcom_file, db_file);
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

pub fn toggle_alias(runcom_file: &str, db_file: &str, alias_name: &str) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let mut alias = match fuzzy_get_alias(alias_name, db_file) {
        Some(alias) => alias,
        None => {
            error!("Alias not found");
            return;
        }
    };

    if alias.name != alias_name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
    }

    alias.enabled = !alias.enabled;
    let _ = match update_alias(&conn, &alias.name, alias.clone()) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not toggle alias");
            return;
        }
    };

    update_runcom(runcom_file, db_file);
    // TODO: change this to success
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

pub fn move_alias_group(runcom_file: &str, db_file: &str, alias_name: &str, group_name: &str) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let mut alias = match fuzzy_get_alias(alias_name, db_file) {
        Some(alias) => alias,
        None => {
            error!("Alias not found");
            return;
        }
    };

    let group = match fuzzy_get_group(db_file, group_name) {
        Some(group) => group,
        None => {
            error!("Group not found");
            return;
        }
    };

    if alias.name != alias_name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
    }

    if group.name != group_name
        && !crate::helpers::questions::yesno!(format!("Did you mean {}?", group.name)).unwrap()
    {
        error!("Please try again with a different group", true);
    }

    alias.group_id = group.id;
    let _ = match update_alias(&conn, &alias.name, alias.clone()) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not move alias group");
            return;
        }
    };

    update_runcom(runcom_file, db_file);
    // TODO: change this to success
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

#[cfg(test)]
mod tests {
    use super::super::add::add_alias;
    use super::*;

    use crate::file_management::database::aliases::get_all_aliases;
    use crate::file_management::database::groups::create_group;
    use crate::file_management::runcom::read_aliases;
    use crate::file_management::Alias;

    #[test]
    fn test_toggle_alias() {
        let db_path = "toggle_alias.db";
        let rc_path = "toggle_alias_rc";
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

        toggle_alias(rc_path, db_path, "test1");

        let truth = vec![Alias {
            name: "test1".to_string(),
            command: r#"echo "test command 1""#.to_string(),
            description: "".to_string(),
            enabled: false,
            group_id: 1,
        }];

        assert_eq!(get_all_aliases(&conn), truth);

        assert_eq!(read_aliases(rc_path).unwrap(), Vec::new());

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
        std::fs::remove_file(rc_path).expect("Error cleaning up test files");
    }

    #[test]
    fn test_rename_alias() {
        let db_path = "rename_alias.db";
        let rc_path = "rename_alias_rc";
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

        rename(rc_path, db_path, "test1", "test2");

        let truth = vec![Alias {
            name: "test2".to_string(),
            command: r#"echo "test command 1""#.to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 1,
        }];

        assert_eq!(get_all_aliases(&conn), truth);

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
        std::fs::remove_file(rc_path).expect("Error cleaning up test files");
    }

    #[test]
    fn test_move_group() {
        let db_path = "move_group_alias.db";
        let rc_path = "move_group_alias_rc";
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

        create_group(&conn, "test_group_1");
        move_alias_group(rc_path, db_path, "test1", "test_group_1");

        let truth = vec![Alias {
            name: "test1".to_string(),
            command: r#"echo "test command 1""#.to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 2,
        }];

        assert_eq!(get_all_aliases(&conn), truth);

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
        std::fs::remove_file(rc_path).expect("Error cleaning up test files");
    }
}
