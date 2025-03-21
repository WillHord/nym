use console::style;

use crate::{error, success};

use super::fuzzy_get_group;

pub fn toggle_group(runcom_file: &str, db_file: &str, group_name: &str) {
    let group = match fuzzy_get_group(db_file, group_name) {
        Some(g) => g,
        None => {
            error!(format!("Could not find group {}", style(group_name).bold()));
            return;
        }
    };

    let enabled =
        group.aliases.iter().all(|a| a.enabled) && group.scripts.iter().all(|g| g.enabled);

    if enabled {
        for alias in group.aliases {
            crate::commands::aliases::edit::toggle_alias(runcom_file, db_file, &alias.name);
        }
        for script in group.scripts {
            crate::commands::scripts::edit::toggle_script(runcom_file, db_file, &script.name);
        }
    } else {
        for alias in group.aliases {
            if !alias.enabled {
                crate::commands::aliases::edit::toggle_alias(runcom_file, db_file, &alias.name);
            }
        }
        for script in group.scripts {
            if !script.enabled {
                crate::commands::scripts::edit::toggle_script(runcom_file, db_file, &script.name);
            }
        }
    }
    success!(format!(
        "Group {} toggled {}",
        style(group.name).bold(),
        if enabled { "off" } else { "on" }
    ))
}
