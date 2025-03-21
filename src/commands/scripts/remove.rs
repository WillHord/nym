use crate::{
    commands::scripts::{confirm_script, fuzzy_get_script},
    error,
    file_management::{
        database::{db_conn, scripts::remove_script as remove_script_from_database},
        update_runcom,
    },
    success,
};

pub fn remove_script(rc_file: &str, db_file: &str, script_name: &str, force: bool) {
    let conn = db_conn(db_file);

    let script = match fuzzy_get_script(db_file, script_name) {
        Some(script) => script,
        None => {
            error!("Script not found");
            return;
        }
    };

    if script.name != script_name && !confirm_script(&script) {
        error!("Please try again with a different script", true);
    }

    if !force
        && !crate::helpers::questions::yesno!(format!(
            "Are you sure you want to delete {}?",
            script.name
        ))
        .unwrap()
    {
        eprintln!("{}", console::style("Exiting").italic());
        std::process::exit(1);
    }

    if remove_script_from_database(&conn, &script.name).is_err() {
        error!("Could not remove script from database");
        return;
    }

    if std::fs::remove_dir_all(
        std::path::Path::new(&script.path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .is_err()
    {
        error!("Could not remove script");
        return;
    }

    update_runcom(rc_file, db_file);
    success!("Script removed successfully");
    println!(
        "Please run {} to activate changes",
        console::style("`exec \"$SHELL\"").bold().italic()
    );
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::scripts::add::add_script, file_management::database::scripts::get_script_by_name,
    };

    use super::*;

    #[test]
    fn remove_script_test() {
        let test_dir = "remove_script_test";
        let db_file = "remove_script_test/nym.db";
        let rc_file = "remove_script_test/nymrc";

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
            "remove_script_test/scripts/test_script/test_script.sh"
        );

        remove_script(rc_file, db_file, "test_script", true);

        let script = get_script_by_name(&conn, "test_script");
        assert!(script.is_err());

        // Clean up
        std::fs::remove_dir_all(test_dir).expect("Error cleaning up test files");
        std::fs::remove_file(script_path).expect("Error cleaning up test files");
    }
}
