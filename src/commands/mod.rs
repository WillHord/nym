pub mod aliases;
pub mod groups;
pub mod scripts;

use std::collections::HashMap;

use crate::file_management::{
    database::{aliases::get_all_aliases, groups::get_group_nameids, scripts::get_all_scripts},
    Alias, Group, Script,
};
use console::style;
use rusqlite::Connection;

#[derive(Clone)]
pub enum Item {
    Alias(Alias),
    Group(Group),
    Script(Script),
}

pub fn get_groups_and_aliases(conn: &Connection) -> Vec<Group> {
    let aliases = get_all_aliases(conn);
    let scripts = get_all_scripts(conn);
    let mut groups = match get_group_nameids(conn) {
        Ok(group_ids) => group_ids,
        Err(_) => return Vec::new(),
    };

    for alias in aliases {
        if let Some(group) = groups.iter_mut().find(|g| g.id == alias.group_id) {
            group.aliases.push(alias);
        }
    }

    for script in scripts {
        if let Some(group) = groups.iter_mut().find(|g| g.id == script.group_id) {
            group.scripts.push(script);
        }
    }

    groups
}

pub fn get_item(db_path: &str, name: &str, include_groups: bool) -> Option<Item> {
    // Fuzzy get item, if not found, return None
    // If multiple exist ask which one should be returned
    let mut item_map = HashMap::new();
    let alias = aliases::fuzzy_get_alias(name, db_path);
    let group = groups::fuzzy_get_group(db_path, name);
    let script = scripts::fuzzy_get_script(db_path, name);

    if let Some(a) = alias {
        if a.name == name {
            return Some(Item::Alias(a));
        }
        item_map.insert(a.name.clone(), Item::Alias(a.clone()));
    }
    if let Some(g) = group {
        if include_groups {
            if g.name == name {
                return Some(Item::Group(g));
            }
            item_map.insert(g.name.clone(), Item::Group(g.clone()));
        }
    }
    if let Some(s) = script {
        if s.name == name {
            return Some(Item::Script(s));
        }
        item_map.insert(s.name.clone(), Item::Script(s.clone()));
    }

    match item_map.len() {
        0 => None,
        1 => {
            // If name is the same return item
            // If not ask if it is the correct item to return
            let item_name = item_map.keys().next().unwrap();
            if item_name == name {
                return Some(item_map[item_name].clone());
            }

            let item_type = match &item_map[item_name] {
                Item::Alias(_) => "alias",
                Item::Group(_) => "group",
                Item::Script(_) => "script",
            };

            if !crate::helpers::questions::yesno!(format!(
                "Did you mean {} ({})?",
                style(item_name).bold(),
                item_type
            ))
            .unwrap()
            {
                return None;
            }
            Some(item_map[item_name].clone())
        }
        _ => {
            // Ask which one to return
            let mut item_names_with_types: Vec<String> = item_map
                .iter()
                .map(|(name, item)| {
                    let item_type = match item {
                        Item::Alias(_) => "alias",
                        Item::Group(_) => "group",
                        Item::Script(_) => "script",
                    };
                    format!("{} ({})", name, item_type)
                })
                .collect();

            item_names_with_types.push("None".to_string());

            let selected_item = inquire::Select::new(
                &format!("Could not file {}, did you mean:", style(name).bold()),
                item_names_with_types,
            )
            .prompt()
            .unwrap();

            if selected_item == "None" {
                return None;
            }

            let selected_item_name = selected_item.split_whitespace().next().unwrap();
            Some(item_map[selected_item_name].clone())
        }
    }
}
