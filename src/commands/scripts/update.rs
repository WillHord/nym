use crate::{
    error,
    file_management::{
        database::{
            db_conn,
            scripts::{add_script as add_script_to_database, get_script_by_name},
        },
        update_runcom, Script,
    },
    helpers::questions::get_filepath,
    success,
};

pub fn update_script(db_file: &str, rc_file: &str, script_name: &str, update_path: Option<&str>) {
    // TODO: Add execution check - if not executable add option to change permissions
    let script_path = match update_path {
        Some(str) => str,
        None => &get_filepath!("Updated script filepath").unwrap(),
    };

    let conn = db_conn(db_file);
    // Check if script exists
    if get_script_by_name(&conn, script_name).is_err() {
        error!("Script does not exists");
        return;
    }

    // get script name from path
    let file_name = script_path
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .to_string();

    let db_path = std::path::Path::new(db_file);
    let parent_dir = db_path.parent().unwrap();

    let scripts_dir = parent_dir.join("scripts");
    let script_name_no_ext = script_name.split('.').collect::<Vec<&str>>()[0];
    if !scripts_dir.exists() && std::fs::create_dir(&scripts_dir).is_err() {
        error!("Issue creating scripts directory");
        return;
    }

    // Create folder in scripts directory
    // if std::fs::create_dir(scripts_dir.join(script_name_no_ext)).is_err() {
    //     error!("Issue creating script directory");
    //     return;
    // }

    // Copy script to scripts directory
    if std::fs::copy(
        script_path,
        scripts_dir
            .join(script_name_no_ext)
            .join(script_name.clone()),
    )
    .is_err()
    {
        error!("Issue copying script to scripts directory");
        return;
    }

    // let script = Script {
    //     name: script_name_no_ext.to_string(),
    //     path: scripts_dir
    //         .join(script_name_no_ext)
    //         .join(script_name.clone())
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    //     description: description.to_string(),
    //     enabled: true,
    //     group_id,
    // };

    // Add script to database
    // if add_script_to_database(&conn, &script).is_err() {
    //     error!("Issue adding script to database");
    //     return;
    // }

    // Update runcom file
    // update_runcom(rc_file, db_file);
    success!("Script added successfully");
    todo!()
}
