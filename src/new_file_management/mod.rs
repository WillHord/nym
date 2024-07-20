pub mod database;
pub mod runcom;

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
