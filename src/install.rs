use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use console::style;
use inquire::Confirm;

use crate::error;
use crate::exit;
use crate::helpers;
use crate::success;
use crate::warning;

fn check_installed(shell_profile: &str) -> bool {
    // Check if the program is already installed
    // If it is, return true
    // If it is not, return false

    // The program is installed if the alias file exists and the source command is in the shell profile file
    let home_dir = dirs::home_dir().unwrap();
    let nymdir = home_dir.join(".nym");
    let nymrc = nymdir.join("nymrc");

    if !nymrc.exists() {
        return false;
    }

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(shell_profile)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains(nymrc.to_str().unwrap()) {
        return true;
    }

    false
}

pub fn install(shell_profile: &str) {
    // OLD/FUTURE: Install program in 3 steps
    // 1. Check the shell and get the shell profile file
    // 1.5 confirm teh shell profile file
    // 2. Check if the progam is already installed
    // 3. Add the program to the shell profile file
    // 3.5 Give user instuctions on how to source the shell profile to enable program

    // Check if shell_profile is valid
    let shell_profile = PathBuf::from(shell_profile);
    if !shell_profile.exists() {
        error!("Shell profile file does not exist", true);
    }

    // Check if the program is already installed
    if check_installed(shell_profile.to_str().unwrap()) {
        error!("Nym is already installed", true);
    }

    // create .nym_aliases file in home directory
    let home_dir = dirs::home_dir().unwrap();
    let nymdir = home_dir.join(".nym");
    let nymrc = nymdir.join("nymrc");
    let nym_db = nymdir.join("nym.db");

    // TODO: Fix this so if no then skip over creating nym dir instead of exiting
    if nymdir.exists()
        && !helpers::questions::yesno!("Alias file already exists. Do you want to overwrite it?")
            .unwrap()
    {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    std::fs::create_dir(nymdir.clone()).expect("Error creating .nym directory");
    std::fs::write(nymrc.clone(), "").expect("Error creating nym config files");
    std::fs::write(nym_db.clone(), "").expect("Error creating nym config files");
    std::fs::create_dir(nymdir.join("scripts")).expect("Error creating scripts directory");

    // Add source command to shell profile file
    let source_command = format!(
        "source {}",
        nymrc.clone().into_os_string().into_string().unwrap()
    );
    let source_command = source_command.as_str();

    // Append source command to shell profile file
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .append(true)
        .open(shell_profile)
        .unwrap();
    let to_write: String = format!("\n# Nym Alias File:\n{}\n", source_command);

    // Check if the source command is already in the shell profile file
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains(&to_write) {
        error!("Nym already installed in shell profile file", true);
    }

    file.write_all(to_write.as_bytes())
        .expect("Error writing to shell profile");

    // set alias file in nymdata
    // crate::file_management::json::set_alias_file(json_file, alias_file.to_str().unwrap());

    success!(format!(
        "Nym installed successfully\nPlease restart your shell to complete the installation {}",
        style("`exec $SHELL`").bold()
    ));
}

// pub fn check_env() -> Shell {
//     // Determine if the user is using bash, zsh, etc.
//     let shell: String = std::env::var("SHELL").unwrap();
//
//     if shell.contains("bash") {
//         println!("Bash detected");
//         return Shell::Bash;
//     } else if shell.contains("zsh") {
//         println!("Zsh detected");
//         return Shell::Zsh;
//     } else {
//         println!("Shell not currently supported");
//         println!("{}", std::env::var("SHELL").unwrap());
//     }
//     Shell::None
// }

pub fn uninstall(shell_profile: &str) {
    // Remove alias file located in json_file.alias_file
    // Remove json file
    // Remove source command from shell profile file located in shell_profile

    // Make sure nym is installed
    if !check_installed(shell_profile) {
        error!("Nym is not installed", true);
    }

    // Ask for confirmation
    if !helpers::questions::yesno!(
        "Are you sure you want to uninstall Nym and delete all aliases created with Nym?"
    )
    .unwrap()
    {
        exit!(1);
    }

    let home_dir = dirs::home_dir().unwrap();
    let nymdir = home_dir.join(".nym");
    let nymrc = nymdir.join("nymrc");

    match std::fs::remove_dir_all(nymdir) {
        Ok(_) => success!("Nym config files were removed successfully"),
        Err(e) => warning!(format!("Failed to remove nym config files: {}", e)),
    };

    // Remove source command from shell profile file
    let source_command: String = format!(
        "# Nym Alias File:\nsource {}",
        nymrc.into_os_string().into_string().unwrap()
    );

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(shell_profile)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if !contents.contains(&source_command) {
        error!(
            format!(
                "Source command not found in shell profile file\n{}",
                source_command
            ),
            true
        );
    }

    file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(shell_profile)
        .unwrap();

    contents = contents.replace(&source_command, "");
    file.write_all(contents.as_bytes())
        .expect("Error writing to shell profile file");

    success!(format!("Nym uninstalled successfully\nPlease restart your shell to complete the uninstallation: {}", style("`exec $SHELL`").bold()));
}
