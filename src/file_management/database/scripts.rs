use rusqlite::{params, Connection};

use crate::file_management::Script;

pub fn add_script(conn: &Connection, script: &Script) -> Result<(), &'static str> {
    if let Err(_x) = conn.execute(
        "INSERT INTO scripts (name, path, description, enabled, group_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            script.name,
            script.path,
            script.description,
            script.enabled,
            script.group_id
        ],
    ) {
        return Err("Error adding script to database");
    }
    Ok(())
}

pub fn get_all_scripts(conn: &Connection) -> Vec<Script> {
    let mut script_query = conn.prepare("SELECT * FROM scripts;").unwrap();

    let mut rows = script_query.query([]).unwrap();
    let mut scripts = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        let name: String = row.get("name").unwrap();
        let path: String = row.get("path").unwrap_or("".to_string());
        let description: String = row.get("description").unwrap_or("".to_string());
        let enabled: bool = row.get("enabled").unwrap();
        let group_id: i32 = row.get("group_id").unwrap();

        scripts.push(Script {
            name,
            path,
            description,
            enabled,
            group_id,
        });
    }
    scripts
}

pub fn get_script_by_name(conn: &Connection, script_name: &str) -> Result<Script, &'static str> {
    let mut script_query = conn
        .prepare("SELECT * FROM scripts WHERE name == (?1);")
        .unwrap();
    let mut rows = script_query.query([script_name]).unwrap();
    if let Some(row) = rows.next().unwrap() {
        Ok(Script {
            name: row.get("name").unwrap(),
            path: row.get("path").unwrap_or("".to_string()),
            description: row.get("description").unwrap_or("".to_string()),
            enabled: row.get("enabled").unwrap(),
            group_id: row.get("group_id").unwrap(),
        })
    } else {
        Err("Script could not be found")
    }
}

pub fn update_script(
    conn: &Connection,
    old_script_name: &str,
    new_script: &Script,
) -> Result<(), &'static str> {
    match conn.execute(
        "UPDATE scripts SET 
        name = (?1),
        path = (?2),
        description = (?3),
        enabled = (?4),
        group_id = (?5)
    WHERE name = (?6)",
        params![
            new_script.name,
            new_script.path,
            new_script.description,
            new_script.enabled,
            new_script.group_id,
            old_script_name
        ],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error updating script"),
    }
}

pub fn remove_script(conn: &Connection, script_name: &str) -> Result<(), &'static str> {
    match conn.execute(
        "DELETE FROM scripts WHERE name == (?1)",
        params![script_name],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error deleting script"),
    }
}

#[cfg(test)]
mod tests {
    use crate::file_management::database::db_conn;

    use super::*;

    #[test]
    fn scripts_sql_test() {
        let db_file = "scripts_sql_test.db";
        let conn = db_conn(db_file);
        assert!(std::path::Path::new(db_file).exists());

        let script = Script {
            name: "test_script".to_string(),
            path: "test_path".to_string(),
            description: "test script".to_string(),
            enabled: true,
            group_id: 1,
        };

        assert!(add_script(&conn, &script).is_ok());

        let script = get_script_by_name(&conn, "test_script");
        assert!(script.is_ok());
        assert_eq!(script.unwrap().name, "test_script");

        let all_scripts = get_all_scripts(&conn);
        assert_eq!(all_scripts.len(), 1);
        assert_eq!(all_scripts[0].name, "test_script");

        let new_script = Script {
            name: "new_test_script".to_string(),
            path: "test_path".to_string(),
            description: "new test script".to_string(),
            enabled: true,
            group_id: 1,
        };

        assert!(update_script(&conn, "test_script", &new_script).is_ok());

        let all_scripts = get_all_scripts(&conn);
        assert_eq!(all_scripts.len(), 1);
        assert_eq!(all_scripts[0].name, "new_test_script");

        let get_script_by_name = get_script_by_name(&conn, "test_script");
        assert!(get_script_by_name.is_err());

        assert!(remove_script(&conn, "new_test_script").is_ok());
        let get_all_scripts = get_all_scripts(&conn);
        assert_eq!(get_all_scripts.len(), 0);

        std::fs::remove_file(db_file).expect("Error cleaning test files");
    }
}
