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
pub struct AliasData {
    pub aliases: Vec<Alias>,
}
