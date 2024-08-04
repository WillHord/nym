use console::style;

use crate::{
    error,
    file_management::database::{
        db_conn,
        groups::{create_group, get_group_by_name},
    },
    success,
};

pub fn add_group(db_file: &str, group_name: &str) {
    let conn = db_conn(db_file);

    if get_group_by_name(&conn, group_name).is_ok() {
        error!(format!(
            "Group with name {} already exists",
            style(group_name).bold()
        ));
        return;
    }

    create_group(&conn, group_name);
    success!(format!("Group {} created successfully", group_name));
}

#[cfg(test)]
mod tests {
    use crate::file_management::{database::groups::get_groups, Group};

    use super::*;

    #[test]
    fn add_group_command_test() {
        let db_path = "add_group_command_test.db";

        let conn = db_conn(db_path);
        add_group(db_path, "Testgroup1");

        let group = get_group_by_name(&conn, "Testgroup1").unwrap();

        assert_eq!(group.name, "Testgroup1");

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
                    name: "Testgroup1".to_string(),
                    aliases: Vec::new(),
                    scripts: Vec::new(),
                },
            ]
        );

        std::fs::remove_file(db_path).expect("Error cleaning up test files");
    }
}
