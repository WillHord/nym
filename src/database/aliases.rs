use clap::error::Result;
use rusqlite::{params, Connection};

use super::NewAlias;

pub fn add_alias(conn: &Connection, alias: &NewAlias) -> Result<(), &'static str> {
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

fn get_all_aliases(conn: &Connection) -> Vec<NewAlias> {
    let mut alias_query = conn.prepare("SELECT * FROM aliases;").unwrap();

    let mut rows = alias_query.query([]).unwrap();
    let mut aliases = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get("name").unwrap();
        let command: String = row.get("command").unwrap();
        let description: String = row.get("description").unwrap_or("".to_string());
        let enabled: bool = row.get("enabled").unwrap();
        let group_id: i32 = row.get("group_id").unwrap();

        aliases.push(NewAlias {
            name,
            command,
            description,
            enabled,
            group_id,
        });
    }
    aliases
}

fn get_alias_by_name(conn: &Connection, name: &str) -> Result<NewAlias, &'static str> {
    let mut alias_query = conn
        .prepare("SELECT * FROM aliases WHERE name == (?1);")
        .unwrap();
    let mut rows = alias_query.query([name]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        Ok(NewAlias {
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
fn remove_alias(conn: &Connection, name: &str) -> Result<(), &'static str> {
    match conn.execute("DELETE FROM aliases WHERE name == (?1)", params![name]) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error deleting alias"),
    }
}
fn update_alias(conn: &Connection, name: &str, updatedAlias: NewAlias) -> Result<(), &'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::super::{setupdb, NewAlias};
    use super::*;

    #[test]
    fn alias_database_test() {
        let test_db = "alias_test.db";
        let conn = match setupdb(test_db) {
            Ok(conn) => conn,
            Err(_) => return,
        };
        assert!(std::path::Path::new(test_db).exists());
        let alias1 = NewAlias {
            name: "alias1".to_string(),
            command: "echo 'test'".to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 1,
        };
        let alias2 = NewAlias {
            name: "alias2".to_string(),
            command: "echo 'test 2'".to_string(),
            description: "description".to_string(),
            enabled: false,
            group_id: 1,
        };

        let _ = add_alias(&conn, &alias1);

        let alias1_get = get_alias_by_name(&conn, "alias1").unwrap();
        assert_eq!(alias1, alias1_get);
        let _ = add_alias(&conn, &alias2);
        let alias2_get = get_alias_by_name(&conn, "alias2").unwrap();
        assert_eq!(alias2, alias2_get);

        let alias_vec = get_all_aliases(&conn);
        assert_eq!(vec![alias1, alias2.clone()], alias_vec);

        let _ = remove_alias(&conn, "alias1");
        let alias_vec = get_all_aliases(&conn);
        assert_eq!(vec![alias2], alias_vec);

        std::fs::remove_file(test_db).expect("Error removing test database");
    }
}
