use crate::file_management::Script;

use super::super::{Alias, Group};
use rusqlite::{params, Connection};

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
            a.enabled as alias_enabled,
            s.name as script_name,
            s.path as script_path,
            s.description as script_description,
            s.enabled as script_enabled
        FROM groups g
        LEFT JOIN aliases a
        ON g.id = a.group_id
        LEFT JOIN scripts s 
        ON g.id = s.group_id;",
        )
        .unwrap();

    let mut group_map = std::collections::HashMap::new();
    let mut rows = alias_query.query([]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let group_id: i32 = row.get("group_id").unwrap();
        let group_name: String = row.get("group_name").unwrap();
        let alias_name: String = row.get("alias_name").unwrap_or("".to_string());
        let alias_command: String = row.get("alias_command").unwrap_or("".to_string());
        let alias_description: String = row.get("description").unwrap_or("".to_string());
        let alias_enabled: bool = row.get("alias_enabled").unwrap_or(false);

        let script_name: String = row.get("script_name").unwrap_or("".to_string());
        let script_path: String = row.get("script_path").unwrap_or("".to_string());
        let script_description: String = row.get("script_description").unwrap_or("".to_string());
        let script_enabled: bool = row.get("script_enabled").unwrap_or(false);

        let group = group_map.entry(group_id).or_insert_with(|| Group {
            id: group_id,
            name: group_name,
            aliases: Vec::new(),
            scripts: Vec::new(),
        });

        if !alias_name.is_empty() && !group.aliases.iter().any(|a| a.name == alias_name) {
            group.aliases.push(Alias {
                name: alias_name,
                command: alias_command,
                description: alias_description,
                enabled: alias_enabled,
                group_id,
            });
        }

        if !script_name.is_empty() && !group.scripts.iter().any(|s| s.name == script_name) {
            group.scripts.push(Script {
                name: script_name,
                path: script_path,
                description: script_description,
                enabled: script_enabled,
                group_id,
            });
        }
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
            scripts: Vec::new(),
        });
    }

    let mut groups = Vec::new();
    for group in group_map.values() {
        groups.push(group.clone());
    }
    groups
}

pub fn get_group_by_name(conn: &Connection, name: &str) -> Result<Group, &'static str> {
    // Get group if exists
    let mut group_query = conn
        .prepare("SELECT * from groups WHERE name = (?1)")
        .unwrap();
    let mut rows = group_query.query([name]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let mut alias_query = conn
            .prepare("SELECT * from aliases WHERE group_id = (?1)")
            .unwrap();
        let group_id: i32 = row.get("id").unwrap();
        let mut aliases = alias_query.query([group_id]).unwrap();
        let mut alias_vec = Vec::new();
        while let Some(alias) = aliases.next().unwrap() {
            alias_vec.push(Alias {
                name: alias.get("name").unwrap(),
                command: alias.get("command").unwrap(),
                description: alias.get("description").unwrap_or("".to_string()),
                enabled: alias.get("enabled").unwrap(),
                group_id,
            })
        }
        Ok(Group {
            id: group_id,
            name: row.get("name").unwrap(),
            aliases: alias_vec,
            scripts: Vec::new(),
        })
    } else {
        Err("Could not file group")
    }
}

pub fn remove_group(conn: &Connection, name: &str) -> Result<(), String> {
    // Remove group
    // If group exists move aliases to uncategorized
    // Do not allow deleting uncategorized (id 1)
    if name == "uncategorized" {
        return Err("Cannot delete uncategorized group".to_string());
    }
    let group = match get_group_by_name(conn, name) {
        Ok(val) => val,
        Err(_) => {
            return Err("Could not find group to remove".to_string());
        }
    };

    let _ = match conn.execute(
        "UPDATE aliases SET group_id = 1 WHERE group_id == (?1)",
        params![group.id],
    ) {
        Ok(val) => val,
        Err(_) => return Err("Error moving aliases to uncategorized".to_string()),
    };

    match conn.execute("DELETE FROM groups WHERE id == (?1)", [group.id]) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Error deleting string: {}", err)),
    }
}

pub fn edit_group(conn: &Connection, group_name: &str, group: Group) -> Result<(), &'static str> {
    match conn.execute(
        "UPDATE groups SET 
            name = (?1)
            WHERE name = (?2)",
        [group.name, group_name.to_string()],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error updating group"),
    }
}

pub fn get_group_nameids(conn: &Connection) -> Result<Vec<Group>, &'static str> {
    let mut group_query = conn.prepare("SELECT * FROM groups;").unwrap();
    let mut rows = group_query.query([]).unwrap();

    let mut groups = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let group_id: i32 = row.get("id").unwrap();
        let name: String = row.get("name").unwrap();

        groups.push(Group {
            id: group_id,
            name,
            aliases: Vec::new(),
            scripts: Vec::new(),
        });
    }

    Ok(groups)
}

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
            scripts: Vec::new(),
        }];
        assert_eq!(curr_groups, base_groups);
        create_group(&conn, "group1");
        let mut curr_groups = get_groups(&conn);
        let mut add_groups: Vec<Group> = base_groups.clone();
        add_groups.push(Group {
            id: 2,
            name: "group1".to_string(),
            aliases: Vec::new(),
            scripts: Vec::new(),
        });
        curr_groups.sort_by(|a, b| a.id.cmp(&b.id));
        assert_eq!(curr_groups, add_groups);

        let uncategorized = get_group_by_name(&conn, "uncategorized").unwrap();
        assert_eq!(*base_groups.first().unwrap(), uncategorized);

        let _ = add_alias(
            &conn,
            &Alias {
                name: "test".to_string(),
                command: "echo \"test\"".to_string(),
                description: "".to_string(),
                enabled: true,
                group_id: 2,
            },
        );

        let group_with_alias = get_group_by_name(&conn, "group1").unwrap();
        let group_with_alias_truth = Group {
            id: 2,
            name: "group1".to_string(),
            aliases: vec![Alias {
                name: "test".to_string(),
                command: "echo \"test\"".to_string(),
                description: "".to_string(),
                enabled: true,
                group_id: 2,
            }],
            scripts: Vec::new(),
        };
        assert_eq!(group_with_alias, group_with_alias_truth);

        remove_group(&conn, "group1").unwrap();
        assert_eq!(
            get_group_by_name(&conn, "uncategorized").unwrap(),
            Group {
                id: 1,
                name: "uncategorized".to_string(),
                aliases: vec![Alias {
                    name: "test".to_string(),
                    command: "echo \"test\"".to_string(),
                    description: "".to_string(),
                    enabled: true,
                    group_id: 1,
                }],
                scripts: Vec::new(),
            }
        );

        std::fs::remove_file("./test.db").expect("Error removing test database");
    }
}
