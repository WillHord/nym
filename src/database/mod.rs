pub mod aliases;
pub mod groups;

use crate::error;
use console::style;
use rusqlite::{params, Connection, Result};

// pub struct Script {
//     pub name: String,
//     pub location: String,
//     pub description: String,
//     pub enabled: bool,
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewAlias {
    pub name: String,
    pub command: String,
    pub description: String,
    pub enabled: bool,
    pub group_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub aliases: Vec<NewAlias>,
}

pub fn setupdb(db_path: &str) -> Result<Connection> {
    let conn = match Connection::open(db_path) {
        Ok(conn) => conn,
        Err(err) => {
            println!("DID NOT WORK");
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
        Ok(_) => println!("Table created"),
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
        Ok(_) => println!("Table created"),
        Err(err) => eprintln!("Error: {}", err),
    };

    let _ = conn.execute(
        "INSERT INTO groups (name) VALUES (?1)",
        params!["uncategorized"],
    );
    Ok(conn)
}
