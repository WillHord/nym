use console::style;

use crate::{
    error,
    file_management::{
        database::{
            db_conn,
            scripts::{add_script as add_script_to_database, get_script_by_name},
        },
        update_runcom, Script,
    },
    success,
};

pub fn add_script(
    rc_file: &str,
    db_file: &str,
    script_path: &str,
    description: &str,
    group_id: i32,
) {
    let conn = db_conn(db_file);
    // Check if script exists
    if let Ok(_x) = get_script_by_name(&conn, script_path) {
        error!("Script already exists");
        return;
    }

    // get script name from path
    let script_name = script_path
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();

    let db_path = std::path::Path::new(db_file);
    let parent_dir = db_path.parent().unwrap();

    let scripts_dir = parent_dir.join("scripts");
    let script_name_no_ext = script_name.split('.').collect::<Vec<&str>>()[0];
    if !scripts_dir.exists() {
        if let Err(_x) = std::fs::create_dir(&scripts_dir) {
            error!("Issue creating scripts directory");
            return;
        }
    }
    // Create folder in scripts directory
    if let Err(_x) = std::fs::create_dir(scripts_dir.join(script_name_no_ext.clone())) {
        error!("Issue creating script directory");
        return;
    }

    // Copy script to scripts directory
    if let Err(_x) = std::fs::copy(
        script_path,
        scripts_dir
            .join(script_name_no_ext)
            .join(script_name.clone()),
    ) {
        error!("Issue copying script to scripts directory");
        return;
    }

    // Add script to database
    if let Err(_x) = add_script_to_database(
        &conn,
        &Script {
            name: script_name_no_ext.to_string(),
            path: scripts_dir
                .join(script_name_no_ext)
                .join(script_name.clone())
                .to_str()
                .unwrap()
                .to_string(),
            description: description.to_string(),
            enabled: true,
            group_id,
        },
    ) {
        error!("Issue adding script to database");
        return;
    }

    // Update runcom file
    // TODO: update runcom to include paths
    // update_runcom(rc_file, db_file);
    success!("Script added successfully");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_script_test() {
        let test_dir = "add_script_test";
        let db_file = "add_script_test/nym.db";
        let rc_file = "add_script_test/nymrc";

        // create test folder
        assert!(std::fs::create_dir(test_dir).is_ok());

        let conn = db_conn(db_file);
        assert!(std::path::Path::new(db_file).exists());

        // Create test script
        let script_path = "test_script.sh";
        let script = r#"echo "test script""#;
        std::fs::write(script_path, script).expect("Error creating test script");

        add_script(rc_file, db_file, script_path, "", 1);

        let script = get_script_by_name(&conn, "test_script");
        assert!(script.is_ok());
        assert_eq!(script.clone().unwrap().name, "test_script");
        assert_eq!(
            script.unwrap().path,
            "add_script_test/scripts/test_script/test_script.sh"
        );

        // Clean up
        std::fs::remove_dir_all(test_dir).expect("Error cleaning up test files");
        std::fs::remove_file(script_path).expect("Error cleaning up test files");
    }
}
