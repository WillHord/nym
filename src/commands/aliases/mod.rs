pub mod add;
pub mod edit;
pub mod list;
pub mod remove;

use crate::{
    helpers::messages::error,
    file_management::{
        database::{aliases::get_all_aliases, setupdb},
        Alias,
    },
};
use console::style;
use fancy_regex::Regex;
use inquire::Confirm;

fn confirm_alias(alias: &Alias) -> bool {
    // Ask for confirmation
    crate::helpers::questions::yesno!(format!("Did you mean {}?", alias.name)).unwrap()
}

pub fn validate_alias(alias: &str) -> bool {
    let pattern = r#"(?:alias\s+)?(\w+)=([\'"])((?:\\.|(?!\2).)*)\2"#;
    let re = Regex::new(pattern).unwrap();

    match re.is_match(alias) {
        Ok(value) => value,
        Err(_) => {
            error!("Error validating alias");
            false
        }
    }
}

pub fn fuzzy_get_alias(name: &str, db_path: &str) -> Option<Alias> {
    // A function to get an alias by name, but also get the closest match if the name doesn't exist
    let conn = match setupdb(db_path) {
        Ok(conn) => conn,
        Err(_) => {
            error!("issue connecting to database");
            return None;
        }
    };
    let aliases = get_all_aliases(&conn);
    let mut closest_match: Option<Alias> = None;
    let mut closest_distance = usize::MAX;

    for alias in aliases {
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

#[test]
fn validate_alias_test() {
    let valid_alias_strings = vec![
        r#"alias alias_name="echo 'test'""#,
        r#"alias_name="echo 'test'""#,
        r#"alias alias_name='echo "test"'"#,
        r#"alias_name='echo "test"'"#,
        r#"alias alias_name="echo \"nested 'test'\"""#,
        r#"alias_name='echo \'nested "test"\'""#,
        r#"alias alias_name="echo \\"test\\"""#,
        r#"alias_name="echo \\"test\\"""#,
    ];

    let invalid_alias_strings = vec![
        r#"alias alias_name = "echo 'test'"#,
        r#"alias alias_name= "echo 'test'"#,
        r#"alias_name= "echo 'test'"#,
        r#"alias alias_name="#,
        r#"alias alias name="echo 'test'"#,
        r#"alias echo 'test'"#,
        r#"alias test="echo 'test'"#,
    ];

    for alias in valid_alias_strings {
        assert!(validate_alias(alias));
    }

    for alias in invalid_alias_strings {
        assert!(!validate_alias(alias));
    }
}
