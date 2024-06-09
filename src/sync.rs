use std::io::Write;
use std::process::exit;

use crate::file_management::aliases::{
    append_alias_to_alias_file, get_aliases_from_alias_file, remove_alias_from_alias_file,
    reset_alias_file, write_aliases_to_alias_file,
};
use crate::file_management::{Alias, AliasData};

pub fn sync_aliases(file: &str, aliases: &[Alias]) {
    // Check for inconsistencies between alias file and json file
}

// pub fn remove_alias(name: &str, file: &str) {
//     remove_alias_from_alias_file(name, file);
// }
//
// pub fn add_alias(alias: &Alias, file: &str) {
//     append_alias_to_alias_file(alias.clone(), file);
// }

pub fn test_source_file(file: &str) {
    // println!("Sourcing file: {}", file);
    // let test = std::process::Command::new(".")
    //     .arg(file)
    //     .output()
    //     .expect("Error sourcing file");

    println!("{}", file);

    let _x = std::io::stdout().flush();

    exit(0)

    // println!("{}", String::from_utf8_lossy(&test.stdout));

    // println!("Sourced file");
}
