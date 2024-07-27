pub mod add;
pub mod list;
pub mod remove;
pub mod rename;

use crate::error;
use crate::file_management::database::aliases::get_group_nameids;
use crate::file_management::{database::setupdb, Group};

use console::style;
use inquire::Confirm;

pub fn fuzzy_get_group(db_path: &str, name: &str) -> Option<Group> {
    // A function to get an alias by name, but also get the closest match if the name doesn't exist
    let conn = match setupdb(db_path) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return None;
        }
    };
    let groups = get_group_nameids(&conn).unwrap();
    let mut closest_match: Option<Group> = None;
    let mut closest_distance = usize::MAX;

    for group in groups {
        if group.name == name {
            return Some(group);
        }

        let distance = strsim::levenshtein(&group.name, name);
        if distance < closest_distance {
            closest_distance = distance;
            closest_match = Some(group);
        }
    }

    closest_match
}

pub fn ask_fuzzy_get(db_path: &str, name: &str) -> Option<Group> {
    let group = match fuzzy_get_group(db_path, name) {
        Some(group) => group,
        None => {
            error!("Group not found");
            std::process::exit(1);
        }
    };
    if group.name != name
        && !crate::helpers::questions::yesno!(format!("Did you mean group: {}?", group.name))
            .unwrap()
    {
        error!("Please try again with a different group", true);
    }

    Some(group)
}
