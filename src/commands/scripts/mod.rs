use crate::file_management::{
    database::{db_conn, scripts::get_all_scripts},
    Script,
};

pub mod add;
pub mod edit;
pub mod list;
pub mod remove;
pub mod update;

pub fn confirm_script(script: &Script) -> bool {
    // Ask for confirmation
    crate::helpers::questions::yesno!(format!("Did you mean {}?", script.name)).unwrap()
}

pub fn fuzzy_get_script(db_path: &str, name: &str) -> Option<Script> {
    let conn = db_conn(db_path);

    let scripts = get_all_scripts(&conn);
    let mut closest_match: Option<Script> = None;
    let mut closest_distance = usize::MAX;

    for script in scripts {
        if script.name == name {
            return Some(script);
        }

        let distance = strsim::levenshtein(&script.name, name);
        if distance < closest_distance {
            closest_distance = distance;
            closest_match = Some(script);
        }
    }
    closest_match
}
