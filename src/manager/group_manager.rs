use crate::{error, helpers, success};
use console::style;

macro_rules! get_group_vector {
    ($conn: expr) => {{
        let mut groups = match crate::file_management::database::groups::get_group_nameids($conn) {
            Ok(g) => g,
            Err(_) => {
                error!("Issue retrieving groups");
                return;
            }
        };

        // Remove uncategorized group from groups vector
        groups.retain(|g| g.name != "uncategorized");

        if groups.is_empty() {
            error!("Could not find any groups to remove");
            return;
        }
        groups
    }};
}

pub fn add_group(db_path: &str) {
    // TODO: Validate group name
    // TODO: Create group name conventions
    let group_name = match inquire::Text::new("Enter new group name:").prompt() {
        Ok(g) => {
            if !g.is_empty() {
                g
            } else {
                error!("Please enter a valid group name");
                return;
            }
        }
        Err(_) => return,
    };

    crate::commands::groups::add::add_group(db_path, &group_name)
}

pub fn bulk_remove_group(rc_file: &str, db_path: &str) {
    let conn = crate::file_management::database::db_conn(db_path);
    let mut groups = match crate::file_management::database::groups::get_group_nameids(&conn) {
        Ok(g) => g,
        Err(_) => {
            error!("Issue retrieving groups");
            return;
        }
    };

    // Remove uncategorized group from groups vector
    groups.retain(|g| g.name != "uncategorized");

    if groups.is_empty() {
        error!("Could not find any groups to remove");
        return;
    }

    let selected_groups = match inquire::MultiSelect::new(
        "Select groups to remove",
        groups.iter().map(|group| &group.name).collect(),
    )
    .prompt()
    {
        Ok(groups) => groups,
        Err(_) => return,
    };

    if selected_groups.is_empty() {
        return;
    }

    if !helpers::questions::yesno!("Are you sure you want to delete these groups (aliases in the groups will be uncategorized)?").unwrap() {
        println!("{}", style("Aborting").yellow());
        return;
    }

    for group in selected_groups {
        crate::commands::groups::remove::remove_group(rc_file, db_path, group, true);
    }

    // TODO: Record if each group was removed then print success "All groups removed" else print the ones that failed
    success!("Groups removed")
}

pub fn rename_group(rc_file: &str, db_file: &str) {
    let conn = crate::file_management::database::db_conn(db_file);

    let groups = get_group_vector!(&conn);

    let selected_group = inquire::Select::new(
        "Select a group to rename",
        groups.iter().map(|g| &g.name).collect(),
    )
    .prompt()
    .unwrap();

    let new_name = inquire::Text::new("Enter the new name:").prompt().unwrap();
    if new_name.is_empty() {
        error!("Group name invalid");
        return;
    }

    crate::commands::groups::rename::rename_group(rc_file, db_file, selected_group, &new_name)
}

pub fn bulk_toggle_group(runcom_file: &str, db_file: &str) {
    let conn = crate::file_management::database::db_conn(db_file);

    let groups = crate::file_management::database::groups::get_groups(&conn);
    if groups.is_empty() {
        error!("Could not find any groups");
        return;
    }

    // If all aliases and scripts in group are enabled then group is (enabled)
    // If all aliases and scripts in group are disabled then group is (disabled)
    // If Some aliases and scripts are enabled then group is (some enabled)
    // Append to group name the proper flag (enabled), (disabled), (some enabled)

    let group_names: Vec<String> = groups
        .iter()
        .map(|group| {
            group.name.clone()
                + " "
                + if group.aliases.iter().all(|a| a.enabled)
                    && group.scripts.iter().all(|a| a.enabled)
                {
                    "(enabled)"
                } else if group.aliases.iter().all(|a| !a.enabled)
                    && group.scripts.iter().all(|a| !a.enabled)
                {
                    "(disabled)"
                } else {
                    "(some enabled)"
                }
        })
        .collect();

    let selected_groups = inquire::MultiSelect::new("Select groups to toggle", group_names)
        .prompt()
        .unwrap();
    if selected_groups.is_empty() {
        return;
    }

    for group in selected_groups {
        let group_name = group.split_whitespace().next().unwrap();
        let enabled_flag = group.split_whitespace().last().unwrap();
        if enabled_flag == "(enabled)" {
            for alias in groups
                .iter()
                .find(|g| g.name == group_name)
                .unwrap()
                .aliases
                .iter()
            {
                crate::commands::aliases::edit::toggle_alias(runcom_file, db_file, &alias.name)
            }
            for script in groups
                .iter()
                .find(|g| g.name == group_name)
                .unwrap()
                .scripts
                .iter()
            {
                crate::commands::scripts::edit::toggle_script(runcom_file, db_file, &script.name)
            }
        } else {
            for alias in groups
                .iter()
                .find(|g| g.name == group_name)
                .unwrap()
                .aliases
                .iter()
            {
                if !alias.enabled {
                    crate::commands::aliases::edit::toggle_alias(runcom_file, db_file, &alias.name)
                }
            }
            for script in groups
                .iter()
                .find(|g| g.name == group_name)
                .unwrap()
                .scripts
                .iter()
            {
                if !script.enabled {
                    crate::commands::scripts::edit::toggle_script(
                        runcom_file,
                        db_file,
                        &script.name,
                    )
                }
            }
        }
    }
}
