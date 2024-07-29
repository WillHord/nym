use crate::file_management::database::{db_conn, scripts::get_all_scripts};

use super::fuzzy_get_script;

pub fn list_scripts(db_file: &str) {
    let conn = db_conn(db_file);

    let scripts = get_all_scripts(&conn);

    if scripts.is_empty() {
        println!("No scripts found");
        return;
    }

    for script in scripts {
        if script.enabled {
            println!("✅ {}", script.name);
        } else {
            println!("❌ {}", script.name);
        }
    }
}

#[allow(dead_code)]
pub fn script_manual(db_file: &str, name: &str) {
    let script = fuzzy_get_script(db_file, name);
    match script {
        Some(script) => {
            if script.name != name {
                println!("Script {} not found showing {}", name, script.name);
            }
            println!("{}: {}", script.name, script.description);
        }
        None => {
            println!("Script {} not found", name);
        }
    }
}
