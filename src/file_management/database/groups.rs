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

        let group = group_map.entry(group_id).or_insert_with(|| Group {
            id: group_id,
            name: group_name,
            aliases: Vec::new(),
        });

        group.aliases.push(Alias {
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
        })
    } else {
        Err("Could not file group")
    }
}

pub fn remove_group(conn: &Connection, name: &str) -> Result<(), &'static str> {
    // Remove group
    // If group exists move aliases to uncategorized
    // Do not allow deleting uncategorized (id 1)
    if name == "uncategorized" {
        return Err("Cannot delete uncategorized group");
    }
    let group = match get_group_by_name(conn, name) {
        Ok(val) => val,
        Err(_) => {
            return Err("Could not find group to remove");
        }
    };

    let _ = match conn.execute(
        "UPDATE aliases SET group_id = 1 WHERE group_id == (?1)",
        params![group.id],
    ) {
        Ok(val) => val,
        Err(_) => return Err("Error moving aliases to uncategorized"),
    };

    match conn.execute("DELETE FROM groups WHERE id == (?1)", params![group.id]) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error deleting group"),
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
        let mut curr_groups = get_groups(&conn);
        let mut add_groups: Vec<Group> = base_groups.clone();
        add_groups.push(Group {
            id: 2,
            name: "group1".to_string(),
            aliases: Vec::new(),
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
                }]
            }
        );

        std::fs::remove_file("./test.db").expect("Error removing test database");
    }
}
