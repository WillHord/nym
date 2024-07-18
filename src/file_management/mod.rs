pub mod aliases;
pub mod json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Alias {
    pub name: String,
    pub command: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct NymData {
    pub aliases: Vec<Alias>,
    pub alias_file: String,
}

// pub struct Script {
//     pub name: String,
//     pub location: String,
//     pub description: String,
//     pub enabled: bool,
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NewAlias {
    pub name: String,
    pub command: String,
    pub description: String,
    pub enabled: bool,
    pub group_id: i32,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub aliases: Vec<NewAlias>,
}
