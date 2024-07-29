use crate::file_management::database::{db_conn, groups::get_groups};

use console::style;
pub fn list_groups(db_file: &str) {
    let conn = db_conn(db_file);

    let groups = get_groups(&conn);

    for group in groups {
        println!("{} - {} aliases", group.name, group.aliases.len());
    }
}

pub fn list_all(db_file: &str) {
    let conn = db_conn(db_file);

    let groups = get_groups(&conn);

    for group in groups {
        println!("{}", style(group.name).bold().underlined());
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
                if script.enabled {
                    println!("\t✅ {}", style(script.name).green());
                } else {
                    println!("\t❌ {}", style(script.name).red());
                }
            }
        }
    }
}
