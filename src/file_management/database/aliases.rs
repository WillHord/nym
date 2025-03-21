use clap::error::Result;
use rusqlite::{params, Connection};

use super::super::Alias;

pub fn add_alias(conn: &Connection, alias: &Alias) -> Result<(), &'static str> {
    let _ = match conn.execute(
        "INSERT INTO aliases (name, command, description, enabled, group_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![alias.name, alias.command, alias.description, alias.enabled, alias.group_id],
    ) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: {}", err);
            return Err("Error creating new alias");
        }
    };
    Ok(())
}

pub fn get_all_aliases(conn: &Connection) -> Vec<Alias> {
    let mut alias_query = conn.prepare("SELECT * FROM aliases;").unwrap();

    let mut rows = alias_query.query([]).unwrap();
    let mut aliases = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get("name").unwrap();
        let command: String = row.get("command").unwrap();
        let description: String = row.get("description").unwrap_or("".to_string());
        let enabled: bool = row.get("enabled").unwrap();
        let group_id: i32 = row.get("group_id").unwrap();

        aliases.push(Alias {
            name,
            command,
            description,
            enabled,
            group_id,
        });
    }
    aliases
}

pub fn get_alias_by_name(conn: &Connection, name: &str) -> Result<Alias, &'static str> {
    let mut alias_query = conn
        .prepare("SELECT * FROM aliases WHERE name == (?1);")
        .unwrap();
    let mut rows = alias_query.query([name]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        Ok(Alias {
            name: row.get("name").unwrap(),
            command: row.get("command").unwrap(),
            description: row.get("description").unwrap_or("".to_string()),
            enabled: row.get("enabled").unwrap(),
            group_id: row.get("group_id").unwrap(),
        })
    } else {
        Err("Error could not find alias")
    }
}

pub fn remove_alias(conn: &Connection, name: &str) -> Result<(), &'static str> {
    match conn.execute("DELETE FROM aliases WHERE name == (?1)", params![name]) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error deleting alias"),
    }
}

pub fn update_alias(
    conn: &Connection,
    old_alias_name: &str,
    updated_alias: Alias,
) -> Result<(), &'static str> {
    match conn.execute(
        "UPDATE aliases SET 
            name = (?1),
            command = (?2),
            description = (?3),
            enabled = (?4),
            group_id = (?5)
        WHERE name = (?6);",
        [
            updated_alias.name,
            updated_alias.command,
            updated_alias.description,
            (updated_alias.enabled as i32).to_string(),
            updated_alias.group_id.to_string(),
            old_alias_name.to_string(),
        ],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error updating alias"),
    }
}

#[cfg(test)]
mod tests {
    use crate::file_management::database::groups::{get_group_nameids, get_groups};
    use crate::file_management::Group;

    use super::super::super::Alias;
    use super::super::setupdb;
    use super::*;

    #[test]
    fn alias_database_test() {
        let test_db = "alias_test.db";
        let conn = match setupdb(test_db) {
            Ok(conn) => conn,
            Err(_) => return,
        };
        assert!(std::path::Path::new(test_db).exists());
        let alias1 = Alias {
            name: "alias1".to_string(),
            command: "echo 'test'".to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 1,
        };
        let alias2 = Alias {
            name: "alias2".to_string(),
            command: "echo 'test 2'".to_string(),
            description: "description".to_string(),
            enabled: false,
            group_id: 1,
        };

        let group_names = get_group_nameids(&conn).unwrap();
        assert_eq!(
            group_names,
            vec![Group {
                id: 1,
                name: "uncategorized".to_string(),
                aliases: Vec::new(),
                scripts: Vec::new(),
            }]
        );

        let _ = add_alias(&conn, &alias1);

        let alias1_get = get_alias_by_name(&conn, "alias1").unwrap();
        assert_eq!(alias1, alias1_get);

        let groups = get_groups(&conn);
        assert_eq!(
            groups,
            vec![Group {
                id: 1,
                name: "uncategorized".to_string(),
                aliases: vec![alias1.clone()],
                scripts: Vec::new(),
            }]
        );

        let _ = add_alias(&conn, &alias2);
        let alias2_get = get_alias_by_name(&conn, "alias2").unwrap();
        assert_eq!(alias2, alias2_get);

        let alias_vec = get_all_aliases(&conn);
        assert_eq!(vec![alias1, alias2.clone()], alias_vec);

        let _ = remove_alias(&conn, "alias1");
        let alias_vec = get_all_aliases(&conn);
        assert_eq!(vec![alias2], alias_vec);

        let updated_alias = Alias {
            name: "alias2".to_string(),
            command: "echo 'updated_alias'".to_string(),
            description: "description".to_string(),
            enabled: true,
            group_id: 1,
        };
        let _ = update_alias(&conn, "alias2", updated_alias.clone());

        let alias_vec = get_all_aliases(&conn);
        assert_eq!(vec![updated_alias], alias_vec);

        std::fs::remove_file(test_db).expect("Error removing test database");
    }
}
