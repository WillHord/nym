use crate::{
    error,
    file_management::{
        database::{db_conn, groups::edit_group},
        update_runcom,
    },
    success,
};
use console::style;

pub fn rename_group(runcom_file: &str, db_file: &str, old_name: &str, new_name: &str) {
    if old_name == "uncategorized" {
        error!("Cannot rename uncategorized group");
        return;
    }

    let conn = db_conn(db_file);

    let mut group =
        match crate::file_management::database::groups::get_group_by_name(&conn, old_name) {
            Ok(group) => group,
            Err(_) => {
                error!(format!("No group with name {}", style(old_name).bold()));
                return;
            }
        };

    group.name = new_name.to_string();
    match edit_group(&conn, old_name, group) {
        Ok(_) => {
            success!("Group successfully renamed");
        }
        Err(_) => {
            error!(format!("could not rename group {}", style(old_name).bold()));
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
    fn rename_group_test() {
        let db_file = "rename_group_test.db";
        let rc_file = "rename_group_test_rc";
        let group_name = "testgroup1";

        let conn = db_conn(db_file);

        add_group(db_file, group_name);

        let group = get_group_by_name(&conn, group_name).unwrap();
        assert_eq!(group.name, group_name);

        rename_group(rc_file, db_file, group_name, "newgroupname");

        let mut group_vec = get_groups(&conn);
        group_vec.sort();
        assert_eq!(
            group_vec,
            vec![
                Group {
                    id: 1,
                    name: "uncategorized".to_string(),
                    aliases: Vec::new(),
                    scripts: Vec::new(),
                },
                Group {
                    id: 2,
                    name: "newgroupname".to_string(),
                    aliases: Vec::new(),
                    scripts: Vec::new(),
                }
            ]
        );

        std::fs::remove_file(db_file).expect("Error cleaning up test files");
        std::fs::remove_file(rc_file).expect("Error cleaning up test files");
    }
}
