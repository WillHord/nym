pub mod aliases;
pub mod groups;
pub mod scripts;

use crate::error;
use rusqlite::{params, Connection, Result};

pub fn setupdb(db_path: &str) -> Result<Connection> {
    let conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(err) => {
            error!("Could not connect to database");
            return Err(err);
        }
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS groups (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    ) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Could not create groups table");
            eprintln!("Error: {}", err);
        }
    };

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS aliases (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            command TEXT NOT NULL,
            description TEXT,
            enabled INTEGER NOT NULL CHECK (enabled IN (0, 1)),
            group_id INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES groups (id) 
        )",
        [],
    ) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {}", err),
    };

    if let Err(err) = conn.execute(
        "CREATE TABLE IF NOT EXISTS scripts (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL,
            description TEXT,
            enabled INTEGER NOT NULL CHECK (enabled IN (0, 1)),
            group_id INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES groups (id)
        )",
        [],
    ) {
        eprintln!("Error: {}", err);
    }

    let _ = conn.execute(
        "INSERT INTO groups (name) VALUES (?1)",
        params!["uncategorized"],
    );
    Ok(conn)
}

pub fn db_conn(db_path: &str) -> Connection {
    match setupdb(db_path) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            std::process::exit(1);
        }
    }
}
