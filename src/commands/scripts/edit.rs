use super::{confirm_script, fuzzy_get_script};
use crate::{
    commands::groups::fuzzy_get_group,
    error,
    file_management::{
        database::{db_conn, scripts::update_script},
        update_runcom, Script,
    },
    success,
};

use console::style;

fn get_script(db_file: &str, script_name: &str) -> Result<Script, &'static str> {
    let script = match fuzzy_get_script(db_file, script_name) {
        Some(script) => script,
        None => return Err("Could not find script"),
    };

    if script.name != script_name && !confirm_script(&script) {
        return Err("Please try again with a different script");
    }
    Ok(script)
}

fn edit_script(
    rc_file: &str,
    db_file: &str,
    old_script: &str,
    new_script: &Script,
    success_msg: String,
) {
    let conn = &db_conn(db_file);
    let _ = match update_script(conn, old_script, &new_script) {
        Ok(_) => true,
        Err(_) => {
            error!("Could not rename script");
            return;
        }
    };

    update_runcom(rc_file, db_file);
    success!(success_msg);
    println!(
        "Please run {} to activate changes",
        style("`exec \"$SHELL\"`").bold().italic()
    );
}

pub fn toggle_script(runcom_file: &str, db_file: &str, script_name: &str) {
    let mut script = match get_script(db_file, script_name) {
        Ok(script) => script,
        Err(e) => {
            error!(format!("{}", e));
            return;
        }
    };

    script.enabled = !script.enabled;

    edit_script(
        runcom_file,
        db_file,
        &script.name,
        &script,
        format!(
            "Script {} is now {}",
            style(&script.name).italic().bold(),
            if script.enabled {
                style("enabled").green()
            } else {
                style("disabled").red()
            }
        ),
    )
}

pub fn rename_script(runcom_file: &str, db_file: &str, old_name: &str, new_name: &str) {
    let mut script = match get_script(db_file, old_name) {
        Ok(script) => script,
        Err(e) => {
            error!(format!("{}", e));
            return;
        }
    };

    // rename script
    // move script to new location reflecting new name
    // update script path in database

    let old_name = script.name;
    script.name = new_name.to_string();

    // Get script file from last item in path
    let script_file = std::path::Path::new(&script.path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let script_ext = script_file.split('.').collect::<Vec<&str>>()[1];
    let new_script_name = if script_ext.is_empty() {
        new_name.to_string()
    } else {
        format!("{}.{}", new_name, script_ext)
    };

    let new_script_path = std::path::Path::new(db_file)
        .parent()
        .unwrap()
        .join("scripts")
        .join(new_name);

    if let Err(err) = std::fs::create_dir(&new_script_path) {
        error!(format!("Issue creating script directory: {}", err));
        return;
    }

    if let Err(err) = std::fs::rename(&script.path, new_script_path.join(new_script_name)) {
        error!(format!("Issue renaming script: {}", err));
        return;
    }

    // remove old name dir
    if let Err(err) = std::fs::remove_dir_all(std::path::Path::new(&script.path).parent().unwrap())
    {
        error!(format!("Issue removing old script directory: {}", err));
        return;
    }

    script.path = new_script_path
        .join(script_file)
        .to_str()
        .unwrap()
        .to_string();

    edit_script(
        runcom_file,
        db_file,
        &old_name,
        &script,
        format!(
            "Script {} has been renamed to {}",
            style(&old_name).bold().italic(),
            style(&script.name).bold().italic()
        ),
    )
}

pub fn move_script(runcom_file: &str, db_file: &str, script_name: &str, group_name: &str) {
    let mut script = match get_script(db_file, script_name) {
        Ok(script) => script,
        Err(e) => {
            error!(format!("{}", e));
            return;
        }
    };

    let group = match fuzzy_get_group(db_file, group_name) {
        Some(group) => group,
        None => {
            error!("Group not found");
            return;
        }
    };

    if group.name != group_name
        && !crate::helpers::questions::yesno!(format!("Did you mean {}?", group.name)).unwrap()
    {
        error!("Please try again with a different group", true);
    }

    script.group_id = group.id;

    edit_script(
        runcom_file,
        db_file,
        &script.name,
        &script,
        format!(
            "Script {} is now in group {}",
            style(&script.name).italic().bold(),
            style(&group.name).bold().underlined()
        ),
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        commands::{groups::add::add_group, scripts::add::add_script},
        file_management::database::scripts::get_script_by_name,
    };

    use super::*;

    #[test]
    fn edit_scripts_test() {
        let test_dir = "edit_script_test";
        let db_file = "edit_script_test/nym.db";
        let rc_file = "edit_script_test/nymrc";

        // create test folder
        assert!(std::fs::create_dir(test_dir).is_ok());

        let conn = db_conn(db_file);
        assert!(std::path::Path::new(db_file).exists());

        // Create test script
        let script_path = "edit_script_test_script.sh";
        let script = r#"echo "test script""#;
        std::fs::write(script_path, script).expect("Error creating test script");

        add_script(rc_file, db_file, script_path, "", 1);

        let script = get_script_by_name(&conn, "edit_script_test_script");
        assert!(script.is_ok());
        assert_eq!(script.clone().unwrap().name, "edit_script_test_script");
        assert_eq!(
            script.unwrap().path,
            "edit_script_test/scripts/edit_script_test_script/edit_script_test_script.sh"
        );

        // Test toggle script
        toggle_script(rc_file, db_file, "edit_script_test_script");
        let script = get_script_by_name(&conn, "edit_script_test_script");
        assert!(script.is_ok());
        assert_eq!(script.clone().unwrap().name, "edit_script_test_script");
        assert!(!script.clone().unwrap().enabled);

        // Test move script
        add_group(db_file, "Group1");
        move_script(rc_file, db_file, "edit_script_test_script", "Group1");
        let script = get_script_by_name(&conn, "edit_script_test_script");
        assert!(script.is_ok());
        assert_eq!(script.clone().unwrap().name, "edit_script_test_script");

        // Test rename script
        rename_script(
            rc_file,
            db_file,
            "edit_script_test_script",
            "new_edit_script_test_script",
        );
        let script = get_script_by_name(&conn, "new_edit_script_test_script");
        assert!(script.is_ok());
        assert_eq!(script.clone().unwrap().name, "new_edit_script_test_script");

        // Check files
        assert!(std::path::Path::new(
            "edit_script_test/scripts/new_edit_script_test_script/new_edit_script_test_script.sh"
        )
        .exists());

        // Clean up
        std::fs::remove_dir_all(test_dir).expect("Error cleaning up test files");
        std::fs::remove_file(script_path).expect("Error cleaning up test files");
    }
}
