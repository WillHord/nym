use crate::{
    error,
    file_management::database::{aliases::get_all_aliases, setupdb},
    warning,
};
use console::style;

use super::fuzzy_get_alias;

pub fn list_aliases(db_file: &str, disabled: bool) {
    let conn = match setupdb(db_file) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return;
        }
    };

    let aliases = get_all_aliases(&conn);

    if aliases.is_empty() {
        warning!("No aliases found");
        return;
    }

    for alias in aliases {
        if alias.enabled && !disabled {
            println!(
                "✅ {}-> {}",
                style(alias.name).green(),
                style(alias.command).green()
            );
        } else if !alias.enabled {
            println!(
                "❌ {} -> {} ",
                style(alias.name).red(),
                style(alias.command).red()
            );
        }
    }
}

pub fn alias_manual(db_file: &str, name: &str) {
    // Print manual for alias
    // Fuzzy get alias, if name not same lsit similar aliases
    // If no similar aliases, print error

    let alias = fuzzy_get_alias(name, db_file);
    match alias {
        Some(alias) => {
            if alias.name != name {
                warning!(format!(
                    "Alias {} not found showing {}",
                    style(name).bold(),
                    style(alias.name.clone()).bold()
                ));
            }
            println!(
                "{}: {}",
                style(alias.name.clone()).bold(),
                alias.description
            );
        }
        None => {
            error!(format!("Alias {} not found", style(name).bold()));
        }
    }
}
