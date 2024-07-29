use crate::file_management::{Alias, Group};
use fancy_regex::Regex;

#[allow(dead_code)]
fn capture_aliases(from_str: &str) -> Vec<String> {
    //TODO: split by groups
    let re = Regex::new(r#"(?:alias\s+)?(\w+)=([\'"])((?:\\.|(?!\2).)*)\2"#).unwrap();
    let mut aliases = Vec::new();

    for cap in re.captures_iter(from_str) {
        if let Some(matched) = cap.unwrap().get(0) {
            aliases.push(matched.as_str().to_string());
        }
    }
    aliases
}

#[allow(dead_code)]
pub fn read_aliases(runcom_file: &str) -> Result<Vec<Alias>, &'static str> {
    let runcom = match std::fs::read_to_string(runcom_file) {
        Ok(runcom) => runcom,
        Err(_) => return Err("Error reading from runcom file"),
    };
    let alias_strings = capture_aliases(&runcom);
    if alias_strings.is_empty() {
        return Ok(Vec::new());
    }

    let mut aliases = Vec::new();

    // Remove beginning "alias " substring
    // Split into name and command;
    for alias_string in alias_strings {
        let line = alias_string[6..].to_string();
        let split: Vec<&str> = line.split('=').collect();
        aliases.push(Alias {
            name: split[0].to_string(),
            command: split[1][1..split[1].len() - 1]
                .to_string()
                .replace('\\', ""),
            description: "".to_string(),
            enabled: true,
            group_id: 0,
        })
    }
    Ok(aliases)
}

pub fn write_to_runcom(runcom_file: &str, groups: Vec<Group>) -> Result<(), &'static str> {
    let mut runcom = String::new();
    runcom.push_str("###############Aliases###############\n");

    // let parent_path = std::path::Path::new(runcom_file).parent().unwrap();
    // let script_dir = parent_path.join("scripts");

    for group in groups {
        runcom.push_str(&format!("\n##########{}##########\n", group.name));
        for alias in group.aliases {
            if alias.enabled {
                // Replace double quotes with \" to escape them
                let alias_command = alias.command.replace('\"', "\\\"");
                runcom.push_str(&format!("alias {}=\"{}\"\n", alias.name, alias_command));
            }
        }

        for script in group.scripts {
            if script.enabled {
                runcom.push_str(&format!("export PATH=$HOME/{}:$PATH\n", script.path));
            }
        }
    }

    match std::fs::write(runcom_file, runcom) {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing to runcom file"),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Alias, Group};
    use super::*;

    #[test]
    fn capture_aliases_test() {
        let capture_from = r#"###############Aliases###############

        ##########uncategorized##########
        alias test_alias_1="echo \"test alias 1\""
        alias test_alias_2="echo \"test alias 2\""

        ##########group1##########
        alias test_alias_3="echo \"test alias 3\"""#;
        let alias_strings = capture_aliases(capture_from);
        assert_eq!(
            alias_strings,
            vec![
                "alias test_alias_1=\"echo \\\"test alias 1\\\"\"".to_string(),
                "alias test_alias_2=\"echo \\\"test alias 2\\\"\"".to_string(),
                "alias test_alias_3=\"echo \\\"test alias 3\\\"\"".to_string(),
            ]
        );
    }

    #[test]
    fn runcom_read_write() {
        let alias1 = Alias {
            name: "test_alias_1".to_string(),
            command: "echo \"test alias 1\"".to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 0,
        };
        let alias2 = Alias {
            name: "test_alias_2".to_string(),
            command: "echo \"test alias 2\"".to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 0,
        };
        let alias3 = Alias {
            name: "test_alias_3".to_string(),
            command: "echo \"test alias 3\"".to_string(),
            description: "".to_string(),
            enabled: true,
            group_id: 0,
        };
        let alias4 = Alias {
            name: "test_alias_4".to_string(),
            command: "echo \"test alias 4\"".to_string(),
            description: "".to_string(),
            enabled: false,
            group_id: 0,
        };

        let group1 = Group {
            id: 1,
            name: "uncategorized".to_string(),
            aliases: vec![alias1.clone(), alias2.clone()],
            scripts: Vec::new(),
        };

        let group2 = Group {
            id: 2,
            name: "group1".to_string(),
            aliases: vec![alias3.clone(), alias4],
            scripts: Vec::new(),
        };

        assert_eq!(Ok(()), write_to_runcom("test1rc", vec![group1, group2]));

        let enabled_aliases = read_aliases("test1rc").unwrap();
        assert_eq!(enabled_aliases, vec![alias1, alias2, alias3]);

        std::fs::remove_file("test1rc").expect("Error deleting test files");
    }
}
