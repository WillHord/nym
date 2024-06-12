use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use console::style;

fn check_installed(shell_profile: &str) -> bool {
    // Check if the program is already installed
    // If it is, return true
    // If it is not, return false

    // The program is installed if the alias file exists and the source command is in the shell profile file
    let home_dir = dirs::home_dir().unwrap();
    let alias_file = home_dir.join(".nym_aliases");
    let alias_file = alias_file.to_str().unwrap();
    if !Path::new(alias_file).exists() {
        return false;
    }

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .open(shell_profile)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if contents.contains(alias_file) {
        return true;
    }

    false
}

pub fn install(json_file: &str, shell_profile: &str) {
    // OLD/FUTURE: Install program in 3 steps
    // 1. Check the shell and get the shell profile file
    // 1.5 confirm teh shell profile file
    // 2. Check if the progam is already installed
    // 3. Add the program to the shell profile file
    // 3.5 Give user instuctions on how to source the shell profile to enable program

    // Check if shell_profile is valid
    let shell_profile = PathBuf::from(shell_profile);
    if !shell_profile.exists() {
        eprintln!(
            "{} Shell profile file does not exist",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }

    // Check if the program is already installed
    if check_installed(shell_profile.to_str().unwrap()) {
        eprintln!("{} Nym is already installed", style("Error:").red().bold());
        std::process::exit(1);
    }

    // create .nym_aliases file in home directory
    let home_dir = dirs::home_dir().unwrap();
    let alias_file = home_dir.join(".nym_aliases");

    // If .alias file already exists, ask user if they want to overwirte it
    if alias_file.exists()
        && !dialoguer::Confirm::new()
            .with_prompt("Alias file already exists. Do you want to overwrite it?")
            .interact()
            .unwrap()
    {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    // Create .nym_aliases file
    std::fs::write(alias_file.clone(), "").expect("Error writing to file");

    // Add source command to shell profile file
    let source_command = format!(
        "source {}",
        alias_file.clone().into_os_string().into_string().unwrap()
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
        eprintln!(
            "{} Source command already in shell profile file",
            style("Error:").red().bold()
        );
        std::process::exit(1);
    }

    file.write_all(to_write.as_bytes())
        .expect("Error writing to shell profile");

    // set alias file in nymdata
    crate::file_management::json::set_alias_file(json_file, alias_file.to_str().unwrap());

    println!(
        "{} Nym installed successfully\nPlease restart you shell to complete the installation: {}",
        style("Success:").green().bold(),
        style("`exec $SHELL`").bold()
    );
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

pub fn uninstall(json_file: &str, shell_profile: &str) {
    // Remove alias file located in json_file.alias_file
    // Remove json file
    // Remove source command from shell profile file located in shell_profile

    // Make sure nym is installed
    if !check_installed(shell_profile) {
        eprintln!("{} Nym is not installed", style("Error:").red().bold());
        std::process::exit(1);
    }

    // Ask for confirmation
    let confirm = dialoguer::Confirm::new()
        .with_prompt(
            "Are you sure you want to uninstall Nym and delete all aliases created with Nym?",
        )
        .interact()
        .unwrap();

    if !confirm {
        eprintln!("{}", style("Exiting").italic());
        std::process::exit(1);
    }

    let alias_file = crate::file_management::json::get_alias_file(json_file);
    let alias_file = alias_file.as_str();

    match std::fs::remove_file(alias_file) {
        Ok(_) => println!(
            "{} Nym aliases file removed successfully.",
            style("Success:").green().bold()
        ),
        Err(e) => eprintln!(
            "{} Failed to remove file: {}",
            style("Warning:").bold().yellow(),
            e
        ),
    }

    match std::fs::remove_file(json_file) {
        Ok(_) => println!(
            "{} Nym config file removed successfully.",
            style("Success:").green().bold()
        ),
        Err(e) => eprintln!(
            "{} Failed to remove file: {}",
            style("Warning:").bold().yellow(),
            e
        ),
    }

    // Remove source command from shell profile file
    let source_command: String = format!("# Nym Alias File:\nsource {}", alias_file);

    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(shell_profile)
        .unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    if !contents.contains(&source_command) {
        eprintln!(
            "{} Source command not found in shell profile file",
            style("Error:").red().bold()
        );
        println!("{}", source_command);
        std::process::exit(1);
    }

    file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(shell_profile)
        .unwrap();

    contents = contents.replace(&source_command, "");
    file.write_all(contents.as_bytes())
        .expect("Error writing to shell profile file");

    println!("{} Nym uninstalled successfully\nPlease restart you shell to complete the uninstallation: {}", style("Success:").green().bold(), style("`exec $SHELL`").bold());
}
