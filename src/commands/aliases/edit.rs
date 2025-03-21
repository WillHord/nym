use crate::{
    commands::{
        aliases::{confirm_alias, fuzzy_get_alias},
        groups::fuzzy_get_group,
    },
    error,
    file_management::{
        database::{aliases::update_alias, db_conn},
        update_runcom, Alias,
    },
    success,
};
use console::style;

// TODO: abstract out connection and fuzzy get alias to private function

fn get_alias(db_file: &str, alias_name: &str) -> Result<Alias, &'static str> {
    let alias = match fuzzy_get_alias(alias_name, db_file) {
        Some(alias) => alias,
        None => return Err("Could not file alias"),
    };

    if alias.name != alias_name && !confirm_alias(&alias) {
        return Err("Please try again with a different alias");
    }
    Ok(alias)
}

fn edit_alias(
    runcom_file: &str,
    db_file: &str,
    old_alias: &str,
    new_alias: &Alias,
    success_msg: String,
) {
    let conn = &db_conn(db_file);
    let _ = match update_alias(conn, old_alias, new_alias.clone()) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not rename alias");
            return;
        }
    };

    update_runcom(runcom_file, db_file);
    success!(success_msg);
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
}

pub fn rename(runcom_file: &str, db_file: &str, old_name: &str, new_name: &str) {
    let mut alias = match get_alias(db_file, old_name) {
        Ok(alias) => alias,
        Err(e) => {
            error!(format!("{}", e));
            return;
        }
    };

    let old_name = alias.name;
    alias.name = new_name.to_string();

    edit_alias(
        runcom_file,
        db_file,
        &old_name,
        &alias,
        format!(
            "Alias {} has been renamed to {}",
            style(old_name.clone()).bold().italic(),
            style(new_name).italic().bold()
        ),
    );
}

pub fn toggle_alias(runcom_file: &str, db_file: &str, alias_name: &str) {
    let mut alias = match get_alias(db_file, alias_name) {
        Ok(alias) => alias,
        Err(e) => {
            error!(format!("{}", e));
            return;
        }
    };
    alias.enabled = !alias.enabled;

    edit_alias(
        runcom_file,
        db_file,
        &alias.name,
        &alias,
        format!(
            "Alias {} is now {}",
            style(alias.clone().name).italic().bold(),
            if alias.enabled {
                style("enabled").bold().green()
            } else {
                style("disabled").bold().red()
            }
        ),
    );
}

pub fn move_alias_group(runcom_file: &str, db_file: &str, alias_name: &str, group_name: &str, _new_group: Option<&str>) {
    let mut alias = match get_alias(db_file, alias_name) {
        Ok(alias) => alias,
        Err(e) => {
            error!(format!("{}", e));
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

    if group.name != group_name
        && !crate::helpers::questions::yesno!(format!("Did you mean {}?", group.name)).unwrap()
    {
        error!("Please try again with a different group", true);
    }

    alias.group_id = group.id;

    edit_alias(
        runcom_file,
        db_file,
        &alias.name,
        &alias,
        format!(
            "Alias {} is now in group {}",
            style(alias.clone().name).italic().bold(),
            style(&group.name).bold().underlined()
        ),
    );
}

#[cfg(test)]
mod tests {
    use super::super::add::add_alias;
    use super::*;

    use crate::file_management::database::aliases::get_all_aliases;
    use crate::file_management::database::groups::create_group;
    use crate::file_management::database::setupdb;
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
        move_alias_group(rc_path, db_path, "test1", "test_group_1", None);

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
