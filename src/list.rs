use crate::file_management::json::get_aliases_from_file;

use termion::color;

pub fn list_aliases(file: &str, disabled: bool) {
    let aliases = get_aliases_from_file(file);
    for alias in aliases.aliases {
        if alias.enabled && !disabled {
            println!(
                "✅ {}{} -> {}",
                color::Fg(color::Green),
                alias.name,
                alias.command
            );
        } else if !alias.enabled {
            println!(
                "❌ {}{} -> {} ",
                color::Fg(color::Red),
                alias.name,
                alias.command
            );
        }
    }
}
