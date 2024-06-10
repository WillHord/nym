use crate::file_management::{Alias, AliasData};

pub fn get_aliases_from_file(file: &str) -> AliasData {
    let json = match std::fs::read_to_string(file) {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    };

    let data: AliasData = match serde_json::from_str(&json) {
        Ok(data) => data,
        Err(_) => AliasData {
            aliases: Vec::new(),
        },
    };

    data
}

pub fn check_alias_exists(name: &str, file: &str) -> bool {
    let aliases = get_aliases_from_file(file);
    aliases.aliases.iter().any(|alias| alias.name == name)
}

pub fn fuzzy_get_alias(name: &str, file: &str) -> Option<Alias> {
    // A function to get an alias by name, but also get the closest match if the name doesn't exist
    let aliases = get_aliases_from_file(file);
    let mut closest_match: Option<Alias> = None;
    let mut closest_distance = usize::MAX;

    for alias in aliases.aliases {
        if alias.name == name {
            return Some(alias);
        }

        let distance = strsim::levenshtein(&alias.name, name);
        if distance < closest_distance {
            closest_distance = distance;
            closest_match = Some(alias);
        }
    }

    closest_match
}

pub fn toggle_alias_by_name(name: &str, file: &str) {
    let mut json = get_aliases_from_file(file);
    for alias in json.aliases.iter_mut() {
        if alias.name == name {
            alias.enabled = !alias.enabled;
        }
    }

    let json = serde_json::to_string_pretty(&json).unwrap();
    std::fs::write(file, json).expect("Error writing to file");
}

pub fn set_alias_by_name(name: &str, enabled: bool, file: &str) {
    let mut json = get_aliases_from_file(file);
    for alias in json.aliases.iter_mut() {
        if alias.name == name {
            alias.enabled = enabled;
        }
    }

    let json = serde_json::to_string_pretty(&json).unwrap();
    std::fs::write(file, json).expect("Error writing to file");
}

pub fn add_alias(alias: &Alias, file: &str) {
    // Get json from file, add new alias to json, write json back to file

    let mut json = match std::fs::read_to_string(file) {
        Ok(json) => json,
        Err(_) => "{}".to_string(),
    };

    if json.is_empty() {
        json = "{}".to_string();
    }

    let mut data: AliasData = match serde_json::from_str(&json) {
        Ok(data) => data,
        Err(_) => AliasData {
            aliases: Vec::new(),
        },
    };

    data.aliases.push(alias.clone());

    let json = serde_json::to_string_pretty(&data).unwrap();
    std::fs::write(file, json).expect("Error writing to file");
}

pub fn remove_alias_by_name(name: &str, file: &str) {
    let mut json = get_aliases_from_file(file);
    json.aliases.retain(|alias| alias.name != name);
    let json = serde_json::to_string_pretty(&json).unwrap();
    std::fs::write(file, json).expect("Error writing to file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_alias() {
        let alias = Alias {
            name: "test".to_string(),
            command: "echo test".to_string(),
            description: "A test alias".to_string(),
            enabled: true,
        };
        let correct = AliasData {
            aliases: vec![alias.clone()],
        };
        add_alias(&alias, "test1.json");
        let contents = get_aliases_from_file("test1.json");

        println!("{:?}", contents);
        assert_eq!(contents, correct);
        std::fs::remove_file("test1.json").unwrap();
    }

    #[test]
    fn test_multiple_writes_to_file() {
        let file: &str = "test2.json";
        let alias1 = Alias {
            name: "test1".to_string(),
            command: "echo test1".to_string(),
            description: "A test alias 1".to_string(),
            enabled: true,
        };

        let alias2 = Alias {
            name: "test2".to_string(),
            command: "echo test2".to_string(),
            description: "A test alias 2".to_string(),
            enabled: true,
        };

        let mut correct = AliasData {
            aliases: vec![alias1.clone()],
        };

        add_alias(&alias1, file);

        let contents = get_aliases_from_file(file);
        assert_eq!(contents, correct);
        add_alias(&alias2, file);
        let contents = get_aliases_from_file(file);
        correct.aliases.push(alias2.clone());
        assert_eq!(contents, correct);

        std::fs::remove_file(file).expect("Failed to remove file");
    }

    #[test]
    fn add_then_remove_alias() {
        let file: &str = "test3.json";
        let alias1 = Alias {
            name: "test1".to_string(),
            command: "echo test1".to_string(),
            description: "A test alias 1".to_string(),
            enabled: true,
        };

        let alias2 = Alias {
            name: "test2".to_string(),
            command: "echo test2".to_string(),
            description: "A test alias 2".to_string(),
            enabled: true,
        };

        let correct1 = AliasData {
            aliases: vec![alias1.clone()],
        };

        let correct2 = AliasData {
            aliases: vec![alias1.clone(), alias2.clone()],
        };

        add_alias(&alias1, file);
        let contents = get_aliases_from_file(file);
        assert_eq!(contents, correct1);
        add_alias(&alias2, file);
        let contents = get_aliases_from_file(file);
        assert_eq!(contents, correct2);
        remove_alias_by_name("test2", file);
        assert_eq!(get_aliases_from_file(file), correct1);

        std::fs::remove_file(file).expect("Failed to remove file");
    }
}
