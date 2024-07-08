use crate::file_management::json::{fuzzy_get_alias, get_aliases_from_file};

use console::style;

pub fn list_aliases(file: &str, disabled: bool) {
    let aliases = get_aliases_from_file(file);
    if aliases.aliases.is_empty() {
        println!(
            "{}: {}",
            style("Warning").yellow().bold(),
            style("No aliases found")
        );
        return;
    }
    for alias in aliases.aliases {
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

pub fn alias_manual(json_file: &str, name: &str) {
    // Print manual for alias
    // Fuzzy get alias, if name not same lsit similar aliases
    // If no similar aliases, print error

    let alias = fuzzy_get_alias(name, json_file);
    match alias {
        Some(alias) => {
            if alias.name != name {
                println!(
                    "{}: Alias {} not found showing {}",
                    style("Warning").yellow().bold(),
                    style(name).bold(),
                    style(alias.name.clone()).bold()
                );
            }
            println!(
                "{}: {}",
                style(alias.name.clone()).bold(),
                alias.description
            );
        }
        None => {
            eprintln!(
                "{}: Alias {} not found",
                style("Error").red().bold(),
                style(name).bold()
            );
        }
    }
}
