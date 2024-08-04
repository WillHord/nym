use super::{confirm_alias, fuzzy_get_alias};
use crate::file_management::database::aliases::remove_alias as remove_alias_from_database;
use crate::file_management::database::setupdb;
use crate::file_management::update_runcom;
use crate::{error, success};
use console::style;

pub fn remove_alias(runcom_file: &str, db_file: &str, alias_name: &str, force: bool) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let alias = match fuzzy_get_alias(alias_name, db_file) {
        Some(alias) => alias,
        None => {
            error!("Alias not found");
            return;
        }
    };

    if alias.name != alias_name && !confirm_alias(&alias) {
        error!("Please try again with a different alias", true);
    }

    if !force
        && !crate::helpers::questions::yesno!(format!(
            "Are you sure you want to delete {}?",
            alias.name
        ))
        .unwrap()
    {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    let _ = match remove_alias_from_database(&conn, &alias.name) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not remove alias");
            return;
        }
    };

    update_runcom(runcom_file, db_file);
    success!("Alias removed successfully");
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
    use crate::file_management::runcom::read_aliases;
    use crate::file_management::Alias;

    #[test]
    fn test_remove_alias() {
        let db_path = "remove_alias.db";
        let rc_path = "remove_alias_rc";
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

        remove_alias(rc_path, db_path, "test1", true);

        assert_eq!(get_all_aliases(&conn), Vec::new());

        assert_eq!(read_aliases(rc_path).unwrap(), Vec::new());

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
        std::fs::remove_file(rc_path).expect("Error cleaning up test files");
    }
}
