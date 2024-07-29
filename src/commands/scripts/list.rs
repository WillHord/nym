use crate::file_management::database::{db_conn, scripts::get_all_scripts};

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
