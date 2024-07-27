use crate::file_management::database::{db_conn, groups::get_groups};

pub fn list_groups(db_file: &str) {
    let conn = db_conn(db_file);

    let groups = get_groups(&conn);

    for group in groups {
        println!("{} - {} aliases", group.name, group.aliases.len());
    }
}
