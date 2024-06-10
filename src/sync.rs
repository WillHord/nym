use console::style;
use dialoguer::Confirm;

use crate::file_management::aliases::{append_alias_to_alias_file, get_aliases_from_alias_file};
use crate::file_management::json::{add_alias, get_aliases_from_file, set_alias_by_name};
use crate::file_management::{Alias, AliasData};

pub fn sync_aliases(json_file: &str, alias_file: &str, force: bool) {
    // Check for inconsistencies between alias file and json file

    let aliases: Vec<Alias> = get_aliases_from_alias_file(alias_file);
    let mut json: AliasData = get_aliases_from_file(json_file);

    // get aliases in aliases file that are not in json file
    let aliases_not_in_json: Vec<Alias> = aliases
        .iter()
        .filter(|alias| !json.aliases.iter().any(|a| a.name == alias.name))
        .cloned()
        .collect();

    if !aliases_not_in_json.is_empty() {
        println!("Found aliases in alias file that are not in json file:");
    }
    for alias in aliases_not_in_json {
        // Ask if user wants to add alias to json file
        if force
            || Confirm::new()
                .with_prompt(format!("Add {} to json file?", alias.name))
                .default(false)
                .interact()
                .unwrap()
        {
            // append_alias_to_alias_file(&alias, json_file);
            add_alias(&alias, json_file);
        }
    }

    // Go through aliases in alias file and ensure they are enabled in json file
    for alias in &aliases {
        if let Some(json_alias) = json.aliases.iter_mut().find(|a| a.name == alias.name) {
            json_alias.enabled = alias.enabled;
            set_alias_by_name(&alias.name, true, json_file);
        }
    }

    // iterate through json file and if there are enabled aliases in the json file
    // that are not in the alias file, ask if user wants to add them to the alias file
    // update json and alias file

    for alias in json.aliases {
        if alias.enabled && !aliases.iter().any(|a| a.name == alias.name) {
            if Confirm::new()
                .with_prompt(format!("Add {} to alias file?", alias.name))
                .default(false)
                .interact()
                .unwrap()
            {
                append_alias_to_alias_file(&alias, alias_file);
            } else {
                // Disable alias in json file
                set_alias_by_name(&alias.name, false, json_file);
            }
        }
    }

    // Print Success message
    println!(
        "{}: Aliases synced successfully",
        style("Success").green().bold()
    );
}

#[cfg(test)]
mod sync_tests {
    use super::*;

    use crate::file_management::aliases::{
        append_alias_to_alias_file, get_aliases_from_alias_file, write_aliases_to_alias_file,
    };
    use crate::file_management::json::get_aliases_from_file;
    use crate::file_management::{Alias, AliasData};

    #[test]
    fn json_alias_mismatch() {
        // Test when alias is in json file (enabled) but not in alias file
        let json_file = "test_json_alias_mismatch.json";
        let alias_file = "test_json_alias_mismatch_alias";

        let alias = Alias {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        let aliases = AliasData {
            aliases: vec![alias.clone()],
        };

        write_aliases_to_alias_file(aliases.aliases.clone(), alias_file);

        sync_aliases(json_file, alias_file, true);

        let aliases = get_aliases_from_alias_file(alias_file);
        let json = get_aliases_from_file(json_file);

        std::fs::remove_file(json_file).expect("Error removing file");
        std::fs::remove_file(alias_file).expect("Error removing file");

        assert_eq!(aliases.len(), 1);
        assert_eq!(json.aliases.len(), 1);
    }

    #[test]
    fn alias_json_mismatch() {
        // Test when alias is in alias file but not in json file
        let json_file = "test_alias_json_mismatch.json";
        let alias_file = "test_alias_json_mismatch_alias";
        let alias = Alias {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        let aliases = AliasData {
            aliases: vec![alias.clone()],
        };

        write_aliases_to_alias_file(aliases.aliases.clone(), alias_file);

        sync_aliases(json_file, alias_file, true);

        let aliases = get_aliases_from_alias_file(alias_file);
        let json = get_aliases_from_file(json_file);

        std::fs::remove_file(json_file).expect("Error removing file");
        std::fs::remove_file(alias_file).expect("Error removing file");
        assert_eq!(aliases.len(), 1);
        assert_eq!(json.aliases.len(), 1);
    }

    #[test]
    fn sync_not_needed() {
        let json_file = "test_sync_not_needed.json";
        let alias_file = "test_sync_not_needed_alias";
        let alias = Alias {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        let aliases = AliasData {
            aliases: vec![alias.clone()],
        };

        write_aliases_to_alias_file(aliases.aliases.clone(), alias_file);
        append_alias_to_alias_file(&alias, json_file);

        sync_aliases(json_file, alias_file, true);

        let aliases = get_aliases_from_alias_file(alias_file);
        let json = get_aliases_from_file(json_file);

        std::fs::remove_file(json_file).expect("Error removing file");
        std::fs::remove_file(alias_file).expect("Error removing file");
        assert_eq!(aliases.len(), 1);
        assert_eq!(json.aliases.len(), 1);
    }
}
