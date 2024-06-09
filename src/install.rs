pub enum Shell {
    Bash,
    Zsh,
    // fish,
    None,
}

pub fn install() {
    // Install program in 3 steps
    // 1. Check the shell and get the shell profile file
    // 1.5 confirm teh shell profile file
    // 2. Check if the progam is already installed
    // 3. Add the program to the shell profile file
    // 3.5 Give user instuctions on how to source the shell profile to enable program

    let shell = check_env();

    match shell {
        Shell::Bash => {
            println!("Bash shell detected");
        }
        Shell::Zsh => {
            println!("Zsh shell detected");
        }
        Shell::None => {
            println!("Shell not supported");
        }
    }
}

pub fn check_env() -> Shell {
    // Determine if the user is using bash, zsh, etc.

    let output = std::process::Command::new("echo")
        .arg("$SHELL")
        .output()
        .expect("Error getting shell");

    let shell = String::from_utf8_lossy(&output.stdout);

    if shell.contains("bash") {
        println!("Bash detected");
        return Shell::Bash;
    } else if shell.contains("zsh") {
        println!("Zsh detected");
        return Shell::Zsh;
    } else {
        println!("Shell not currently supported");
    }
    Shell::None
}
