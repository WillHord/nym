use console::style;

use crate::{error, helpers};

pub fn bulk_toggle_scripts(runcom_file: &str, db_file: &str) {
    let conn = crate::file_management::database::db_conn(db_file);
    let scripts = crate::file_management::database::scripts::get_all_scripts(&conn);
    if scripts.is_empty() {
        error!("Could not find any scripts");
        return;
    }

    let script_names: Vec<String> = scripts.iter().map(|s| s.name.clone()).collect();
    let selected_scripts = inquire::MultiSelect::new("Select scripts to toggle", script_names)
        .prompt()
        .unwrap();
    if selected_scripts.is_empty() {
        return;
    }

    for script in selected_scripts {
        crate::commands::scripts::edit::toggle_script(runcom_file, db_file, &script);
    }
}

pub fn add_script(runcom_file: &str, db_file: &str) {
    // TODO: Implement this - auto complete script path
    todo!()
}

pub fn bulk_remove_scripts(runcom_file: &str, db_file: &str) {
    let conn = crate::file_management::database::db_conn(db_file);
    let scripts = crate::file_management::database::scripts::get_all_scripts(&conn);
    if scripts.is_empty() {
        error!("Could not find any scripts");
        return;
    }

    let script_names: Vec<String> = scripts.iter().map(|s| s.name.clone()).collect();
    let selected_scripts = inquire::MultiSelect::new("Select scripts to remove", script_names)
        .prompt()
        .unwrap();
    if selected_scripts.is_empty() {
        return;
    }

    if !helpers::questions::yesno!("Are you sure you want to remove these scripts?").unwrap() {
        println!("{}", style("Aborting").yellow());
        return;
    }

    for script in selected_scripts {
        crate::commands::scripts::remove::remove_script(runcom_file, db_file, &script, true);
    }
}

pub fn rename_script(runcom_file: &str, db_file: &str) {
    let conn = crate::file_management::database::db_conn(db_file);
    let scripts = crate::file_management::database::scripts::get_all_scripts(&conn);
    if scripts.is_empty() {
        error!("Could not find any scripts");
        return;
    }

    let selected_script = inquire::Select::new(
        "Select script to rename",
        scripts.iter().map(|s| &s.name).collect(),
    )
    .prompt()
    .unwrap();

    let new_name = inquire::Text::new("Enter the new name:").prompt().unwrap();
    if new_name.is_empty() {
        error!("Script name invalid");
        return;
    }

    crate::commands::scripts::edit::rename_script(runcom_file, db_file, selected_script, &new_name);
}
