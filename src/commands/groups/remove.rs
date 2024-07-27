use console::style;
use inquire::Confirm;

use crate::{
    error,
    file_management::{
        database::{db_conn, groups::remove_group as remove_group_database},
        update_runcom,
    },
    success,
};

use super::fuzzy_get_group;

pub fn remove_group(runcom_file: &str, db_file: &str, group_name: &str, force: bool) {
    let conn = db_conn(db_file);

    let group = match fuzzy_get_group(db_file, group_name) {
        Some(group) => group,
        None => {
            error!("Group not found");
            return;
        }
    };

    if group.name != group_name
        && !crate::helpers::questions::yesno!(format!("Did you mean group: {}?", group.name))
            .unwrap()
    {
        error!("Please try again with a different group", true);
    }

    if !force
        && !crate::helpers::questions::yesno!(format!(
            "Are you sure you want to delete the group {}?",
            group.name
        ))
        .unwrap()
    {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    match remove_group_database(&conn, &group.name) {
        Ok(_) => success!("Group successfully deleted"),
        Err(_) => {
            error!(format!(
                "could not delete group {}",
                style(group.name).bold()
            ));
            return;
        }
    };

    update_runcom(runcom_file, db_file);
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::groups::add::add_group,
        file_management::{
            database::groups::{get_group_by_name, get_groups},
            Group,
        },
    };

    use super::*;

    #[test]
    fn remove_group_test() {
        let db_file = "remove_group_test.db";
        let rc_file = "remove_group_test_rc";
        let group_name = "testgroup1";

        let conn = db_conn(db_file);

        add_group(db_file, group_name);

        let group = get_group_by_name(&conn, group_name).unwrap();

        assert_eq!(group.name, group_name);

        remove_group(rc_file, db_file, group_name, true);
        let group_vec = get_groups(&conn);
        assert_eq!(
            group_vec,
            vec![Group {
                id: 1,
                name: "uncategorized".to_string(),
                aliases: Vec::new(),
            }]
        );

        std::fs::remove_file(db_file).expect("Error cleaning up test files");
        std::fs::remove_file(rc_file).expect("Error cleaning up test files");
    }
}
