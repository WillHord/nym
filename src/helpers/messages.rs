#[macro_export]
macro_rules! success {
    ($message: expr) => {
        println!("{}: {}", style("Success").bold().green(), $message)
    };
}

#[macro_export]
macro_rules! info {
    ($message: expr) => {
        println!("{}: {}", style("Info").bold().cyan(), $message)
    };
}

#[macro_export]
macro_rules! error {
    ($message: expr) => {
        eprintln!("{}: {}", style("Error").bold().red(), $message)
    };
    ($message: expr, $exit: expr) => {
        eprintln!("{}: {}", style("Error").bold().red(), $message);
        if $exit {
            std::process::exit(1);
        }
    };
}

#[macro_export]
macro_rules! warning {
    ($message: expr) => {
        println!("{}: {}", style("Warning").bold().yellow(), $message)
    };
}

#[macro_export]
macro_rules! exit {
    ($status: literal) => {
        if $status != 0 {
            eprintln!("{}", style("Exiting").italic());
        } else {
            println!("{}", style("Exiting").italic());
        }
        std::process::exit($status);
    };
}

pub(crate) use error;
pub(crate) use success;
pub(crate) use warning;
