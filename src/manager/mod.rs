mod alias_manager;
mod group_manager;
mod script_manager;

use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};

use indexmap::IndexMap;
use std::rc::Rc;

#[derive(Clone)]
enum MenuItem {
    Function(Rc<dyn Fn(&Context)>),
    SubMenu(IndexMap<&'static str, MenuItem>),
}

struct Context {
    rc_file: String,
    db_file: String,
}

pub fn start_manager(rc_file: &str, db_file: &str) {
    let mut main_menu = IndexMap::new();

    let context = Rc::new(Context {
        rc_file: rc_file.to_string(),
        db_file: db_file.to_string(),
    });

    let mut alias_menu = IndexMap::new();
    alias_menu.insert(
        "List aliases",
        MenuItem::Function(Rc::new(|ctx| {
            crate::commands::aliases::list::list_aliases(&ctx.db_file, false)
        })),
    );
    alias_menu.insert(
        "Add alias",
        MenuItem::Function(Rc::new(|ctx| {
            alias_manager::add_alias(&ctx.rc_file, &ctx.db_file)
        })),
    );
    alias_menu.insert(
        "Remove aliases",
        MenuItem::Function(Rc::new(|ctx| {
            alias_manager::bulk_remove_aliases(&ctx.rc_file, &ctx.db_file)
        })),
    );
    alias_menu.insert(
        "Rename alias",
        MenuItem::Function(Rc::new(|ctx| {
            alias_manager::rename_alias(&ctx.rc_file, &ctx.db_file)
        })),
    );
    alias_menu.insert(
        "Toggle aliases",
        MenuItem::Function(Rc::new(|ctx| {
            alias_manager::bulk_toggle_aliases(&ctx.rc_file, &ctx.db_file)
        })),
    );
    // TODO: Add move alias
    //

    let mut script_menu = IndexMap::new();
    script_menu.insert(
        "List scripts",
        MenuItem::Function(Rc::new(|ctx| {
            crate::commands::scripts::list::list_scripts(&ctx.db_file)
        })),
    );
    // script_menu.insert("Add script", MenuItem::Function(|| {}));
    script_menu.insert(
        "Remove script",
        MenuItem::Function(Rc::new(|ctx| {
            script_manager::bulk_remove_scripts(&ctx.rc_file, &ctx.db_file)
        })),
    );
    script_menu.insert(
        "Rename script",
        MenuItem::Function(Rc::new(|ctx| {
            script_manager::rename_script(&ctx.rc_file, &ctx.db_file)
        })),
    );
    script_menu.insert(
        "Toggle script",
        MenuItem::Function(Rc::new(|ctx| {
            script_manager::bulk_toggle_scripts(&ctx.rc_file, &ctx.db_file)
        })),
    );

    let mut group_menu = IndexMap::new();
    group_menu.insert(
        "List groups",
        MenuItem::Function(Rc::new(|ctx| {
            crate::commands::groups::list::list_groups(&ctx.db_file)
        })),
    );
    group_menu.insert(
        "Add group",
        MenuItem::Function(Rc::new(|ctx| group_manager::add_group(&ctx.db_file))),
    );
    group_menu.insert(
        "Remove groups",
        MenuItem::Function(Rc::new(|ctx| {
            group_manager::bulk_remove_group(&ctx.rc_file, &ctx.db_file)
        })),
    );
    group_menu.insert(
        "Rename group",
        MenuItem::Function(Rc::new(|ctx| {
            group_manager::rename_group(&ctx.rc_file, &ctx.db_file)
        })),
    );
    group_menu.insert(
        "Toggle group",
        MenuItem::Function(Rc::new(|ctx| {
            group_manager::bulk_toggle_group(&ctx.rc_file, &ctx.db_file)
        })),
    );

    main_menu.insert("Aliases", MenuItem::SubMenu(alias_menu));
    main_menu.insert("Scripts", MenuItem::SubMenu(script_menu));
    main_menu.insert("Groups", MenuItem::SubMenu(group_menu));
    main_menu.insert(
        "Quit",
        MenuItem::Function(Rc::new(|_| {
            std::process::exit(0);
        })),
    );

    handle_menu(main_menu, None, context);
}

fn handle_menu(
    menu: IndexMap<&str, MenuItem>,
    parent_menu: Option<IndexMap<&str, MenuItem>>,
    context: Rc<Context>,
) {
    let mut options: Vec<&str> = menu.keys().cloned().collect();

    if parent_menu.is_some() {
        options.push("Back");
    }

    loop {
        let selection = match inquire::Select::new("Select an option", options.clone()).prompt() {
            Ok(selection) => selection,
            Err(_) => {
                println!("Exiting nym");
                std::process::exit(0);
            }
        };

        execute!(
            stdout(),
            cursor::MoveToPreviousLine(1),
            terminal::Clear(ClearType::CurrentLine)
        )
        .unwrap();
        stdout().flush().unwrap();

        if selection == "Back" {
            if let Some(parent) = parent_menu {
                handle_menu(parent, None, context.clone()); // Go back to parent menu
            }
            break;
        }

        match menu.get(selection) {
            Some(MenuItem::Function(f)) => f(&context),
            Some(MenuItem::SubMenu(sub_menu)) => {
                handle_menu(sub_menu.clone(), Some(menu.clone()), context.clone())
            }
            None => println!("Invalid selection"),
        }
    }
}
