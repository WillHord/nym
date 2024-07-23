use database::{aliases::get_groups_and_aliases, setupdb};
use runcom::write_to_runcom;

use crate::error;
use console::style;

pub mod database;
pub mod runcom;

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

pub fn update_runcom(runcom_file: &str, db_file: &str) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue conncecting to database");
            return;
        }
    };

    let groups = get_groups_and_aliases(&conn);
    for g in groups.clone() {
        println!("!group len {}", g.aliases.len());
    }
    match write_to_runcom(runcom_file, groups) {
        Ok(_) => (),
        Err(_) => {
            error!("issue writing aliases to rc file");
        }
    }
}
