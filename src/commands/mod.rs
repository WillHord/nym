pub mod aliases;
pub mod groups;
pub mod scripts;

use crate::file_management::{
    database::{aliases::get_all_aliases, groups::get_group_nameids, scripts::get_all_scripts},
    Group,
};
use rusqlite::Connection;

pub fn get_groups_and_aliases(conn: &Connection) -> Vec<Group> {
    let aliases = get_all_aliases(conn);
    let scripts = get_all_scripts(conn);
    let mut groups = match get_group_nameids(conn) {
        Ok(group_ids) => group_ids,
        Err(_) => return Vec::new(),
    };

    for alias in aliases {
        if let Some(group) = groups.iter_mut().find(|g| g.id == alias.group_id) {
            group.aliases.push(alias);
        }
    }

    for script in scripts {
        if let Some(group) = groups.iter_mut().find(|g| g.id == script.group_id) {
            group.scripts.push(script);
        }
    }

    groups
}
