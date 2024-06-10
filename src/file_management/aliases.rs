use crate::file_management::Alias;

pub fn get_aliases_from_alias_file(file: &str) -> Vec<Alias> {
    let alias_dotfile = match std::fs::read_to_string(file) {
        Ok(alias_dotfile) => alias_dotfile,
        Err(_) => "".to_string(),
    };

    // Aliases in dotfile should be in format: alias alias_name="command"\n
    // Aliases can have either '' or "" around the command
    // Split on newline to get each alias
    let aliases: Vec<Alias> = alias_dotfile
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            // remove leading "alias"
            let line = line[6..].to_string();
            let alias_data: Vec<&str> = line.split('=').collect();
            Alias {
                name: alias_data[0].to_string(),
                command: alias_data[1][1..alias_data[1].len() - 1]
                    .to_string()
                    .replace('\\', ""),
                description: "".to_string(),
                enabled: true,
            }
        })
        .collect();

    aliases
}

// pub fn reset_alias_file(file: &str) {
//     std::fs::remove_file(file).expect("Error removing file");
//     std::fs::write(file, "").expect("Error writing to file");
// }

pub fn write_aliases_to_alias_file(aliases: Vec<Alias>, file: &str) {
    let mut alias_dotfile = String::new();

    for alias in aliases {
        // Replace double quotes with \" to escape them
        let alias_command = alias.command.replace('\"', "\\\"");
        alias_dotfile.push_str(&format!("alias {}=\"{}\"\n", alias.name, alias_command));
    }

    std::fs::write(file, alias_dotfile).expect("Error writing to file");
}

pub fn append_alias_to_alias_file(alias: &Alias, file: &str) {
    let mut aliases = get_aliases_from_alias_file(file);
    aliases.push(alias.clone());
    write_aliases_to_alias_file(aliases, file);
}

// TODO: Return bool to check if alias was removed or not
pub fn remove_alias_from_alias_file(name: &str, file: &str) {
    let mut aliases = get_aliases_from_alias_file(file);
    aliases.retain(|alias| alias.name != name);
    write_aliases_to_alias_file(aliases, file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_write_read_alias() {
        let file: &str = "test_alias_file1";

        let alias = Alias {
            name: "test".to_string(),
            command: "echo \"test\"".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        append_alias_to_alias_file(&alias.clone(), file);
        let aliases = get_aliases_from_alias_file(file);
        std::fs::remove_file(file).expect("Error removing file");
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].name, alias.name);
        assert_eq!(aliases[0].command, alias.command);
    }

    #[test]
    fn test_multiple_write_read_alias() {
        let file: &str = "test_alias_file2";

        let alias1 = Alias {
            name: "test1".to_string(),
            command: "echo test1".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        let alias2 = Alias {
            name: "test2".to_string(),
            command: "echo test2".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        append_alias_to_alias_file(&alias1.clone(), file);
        let aliases = get_aliases_from_alias_file(file);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].name, alias1.name);
        assert_eq!(aliases[0].command, alias1.command);

        append_alias_to_alias_file(&alias2.clone(), file);
        let aliases = get_aliases_from_alias_file(file);
        assert_eq!(aliases.len(), 2);
        assert_eq!(aliases[0].name, alias1.name);
        assert_eq!(aliases[0].command, alias1.command);
        assert_eq!(aliases[1].name, alias2.name);
        assert_eq!(aliases[1].command, alias2.command);

        std::fs::remove_file(file).expect("Error removing file");
    }

    #[test]
    fn test_multiple_write_and_remove_alias() {
        let file: &str = "test_alias_file3";

        let alias1 = Alias {
            name: "test1".to_string(),
            command: "echo test1".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        let alias2 = Alias {
            name: "test2".to_string(),
            command: "echo test2".to_string(),
            description: "".to_string(),
            enabled: true,
        };

        append_alias_to_alias_file(&alias1.clone(), file);
        let aliases = get_aliases_from_alias_file(file);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].name, alias1.name);
        assert_eq!(aliases[0].command, alias1.command);

        append_alias_to_alias_file(&alias2.clone(), file);
        let aliases = get_aliases_from_alias_file(file);
        assert_eq!(aliases.len(), 2);
        assert_eq!(aliases[0].name, alias1.name);
        assert_eq!(aliases[0].command, alias1.command);
        assert_eq!(aliases[1].name, alias2.name);
        assert_eq!(aliases[1].command, alias2.command);

        remove_alias_from_alias_file(&alias1.name, file);
        let aliases = get_aliases_from_alias_file(file);
        assert_eq!(aliases.len(), 1);
        assert_eq!(aliases[0].name, alias2.name);
        assert_eq!(aliases[0].command, alias2.command);

        std::fs::remove_file(file).expect("Error removing file");
    }
}
