use database::{aliases::get_groups_and_aliases, setupdb};
use runcom::write_to_runcom;

use crate::error;
use console::style;

pub mod database;
pub mod runcom;

// #[allow(dead_code)]
// #[derive(Debug)]
// pub enum ScriptType {
//     Executable,
//     Python,
// }

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Script {
    pub name: String,
    pub path: String,
    pub description: String,
    pub enabled: bool,
    pub group_id: i32,
    // pub type_: ScriptType,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: String,
    pub enabled: bool,
    pub group_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub aliases: Vec<Alias>,
    pub scripts: Vec<Script>,
}

pub fn update_runcom(runcom_file: &str, db_file: &str) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue conncecting to database");
            return;
        }
    };

    let groups = get_groups_and_aliases(&conn);
    match write_to_runcom(runcom_file, groups) {
        Ok(_) => (),
        Err(_) => {
            error!("issue writing aliases to rc file");
        }
    }
}
