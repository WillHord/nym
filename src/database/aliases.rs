use rusqlite::{params, Connection, Result};

use super::NewAlias;

pub fn add_alias(conn: &Connection, alias: &NewAlias) {
    let _ = match conn.execute(
        "INSERT INTO aliases (name, command, description, enabled, group_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![alias.name, alias.command, alias.description, alias.enabled, alias.group_id],
    ) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
}

// fn get_all_aliases(conn: &conn) -> Vec<Alias> {}
// fn get_alias_by_name() {}
