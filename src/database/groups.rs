use rusqlite::{params, Connection, Result};

use crate::error;
use console::style;

use super::{Group, NewAlias};

pub fn create_group(conn: &Connection, name: &str) {
    let _ = match conn.execute("INSERT INTO groups (name) VALUES (?1)", params![name]) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERROR: {}", err);
            return;
        }
    };
}

pub fn get_groups(conn: &Connection) -> Vec<Group> {
    let mut alias_query = conn
        .prepare(
            " SELECT
            g.id as group_id,
            g.name as group_name,
            a.id as alias_id,
            a.name as alias_name,
            a.command as alias_command,
            a.description as alias_description,
            a.enabled as alias_enabled
        FROM groups g
        RIGHT JOIN aliases a
        ON g.id = a.group_id;",
        )
        .unwrap();

    let mut group_map = std::collections::HashMap::new();
    let mut rows = alias_query.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let group_id: i32 = row.get("group_id").unwrap();
        let group_name: String = row.get("group_name").unwrap();
        // let alias_id: i32 = row.get("alias_id").unwrap();
        let alias_name: String = row.get("alias_name").unwrap();
        let alias_command: String = row.get("alias_command").unwrap();
        let alias_description: String = row.get("description").unwrap_or("".to_string());
        let alias_enabled: bool = row.get("alias_enabled").unwrap();
        println!("alias: {}", alias_name);

        let group = group_map.entry(group_id).or_insert_with(|| Group {
            id: group_id,
            name: group_name,
            aliases: Vec::new(),
        });

        group.aliases.push(NewAlias {
            name: alias_name,
            command: alias_command,
            description: alias_description,
            enabled: alias_enabled,
            group_id,
        });
    }
    let mut group_query = conn.prepare("SELECT * FROM groups;").unwrap();
    let mut rows = group_query.query([]).unwrap();
    while let Some(row) = rows.next().unwrap() {
        let group_id: i32 = row.get("id").unwrap();
        let group_name: String = row.get("name").unwrap();

        let _ = group_map.entry(group_id).or_insert_with(|| Group {
            id: group_id,
            name: group_name,
            aliases: Vec::new(),
        });
    }

    let mut groups = Vec::new();
    for group in group_map.values() {
        groups.push(group.clone());
    }
    groups
}
fn get_group_by_name() {}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::aliases::add_alias;
    use super::super::setupdb;

    #[test]
    fn groups_database_test() {
        let conn = match setupdb("./test.db") {
            Ok(conn) => conn,
            Err(_) => return,
        };
        assert!(std::path::Path::new("./test.db").exists());
        let curr_groups = get_groups(&conn);
        let base_groups = vec![Group {
            id: 1,
            name: "uncategorized".to_string(),
            aliases: Vec::new(),
        }];
        assert_eq!(curr_groups, base_groups);
        create_group(&conn, "group1");
        let curr_groups = get_groups(&conn);
        let mut add_groups: Vec<Group> = base_groups.clone();
        add_groups.push(Group {
            id: 2,
            name: "group1".to_string(),
            aliases: Vec::new(),
        });
        assert_eq!(curr_groups, add_groups);

        add_alias(
            &conn,
            &NewAlias {
                name: "test".to_string(),
                command: "echo \"test\"".to_string(),
                description: "".to_string(),
                enabled: true,
                group_id: 1,
            },
        );

        std::fs::remove_file("./test.db").expect("Error removing test database");
    }
}
