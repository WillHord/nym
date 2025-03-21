use crate::file_management::database::{db_conn, groups::get_groups};

use console::style;
pub fn list_groups(db_file: &str) {
    let conn = db_conn(db_file);

    let groups = get_groups(&conn);

    for group in groups {
        println!("{}", style(group.name).bold());
        if group.aliases.is_empty() && group.scripts.is_empty() {
            println!("    {}", style("Empty").dim());
        }
        if !group.aliases.is_empty() {
            println!("    {} Aliases", group.aliases.len());
        }
        if !group.scripts.is_empty() {
            println!("    {} Scripts", group.scripts.len());
        }
    }
}

pub fn list_all(db_file: &str) {
    let conn = db_conn(db_file);

    let groups = get_groups(&conn);

    for group in groups {
        println!("{}:", style(group.name).bold().underlined());
        if group.aliases.is_empty() && group.scripts.is_empty() {
            println!("    {}", style("Empty").dim());
        }
        if !group.aliases.is_empty() {
            println!("    Aliases:");
            for alias in group.aliases {
                if alias.enabled {
                    println!(
                        "\t✅ {}-> {}",
                        style(alias.name).green(),
                        style(alias.command).green()
                    );
                } else {
                    println!(
                        "\t❌ {} -> {} ",
                        style(alias.name).red(),
                        style(alias.command).red()
                    );
                }
            }
        }
        if !group.scripts.is_empty() {
            println!("    Scripts:");
            for script in group.scripts {
                // TODO: Save name with extension for easier printing
                let script_file = std::path::Path::new(&script.path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                if script.enabled {
                    println!("\t✅ {}", style(script_file).green());
                } else {
                    println!("\t❌ {}", style(script_file).red());
                }
            }
        }
    }
}
