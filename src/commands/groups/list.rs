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
        for alias in group.aliases {
            if alias.enabled {
                println!(
                    "✅ {}-> {}",
                    style(alias.name).green(),
                    style(alias.command).green()
                );
            } else {
                println!(
                    "❌ {} -> {} ",
                    style(alias.name).red(),
                    style(alias.command).red()
                );
            }
        }
        println!();
    }
}
