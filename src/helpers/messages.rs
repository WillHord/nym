#[macro_export]
macro_rules! success {
    ($message: expr) => {
        println!("{}: {}", style("Success").bold().green(), $message);
    };
}

#[macro_export]
macro_rules! error {
    ($message: expr) => {
        println!("{}: {}", style("Error").bold().red(), $message);
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
        println!("{}: {}", style("Warning").bold().yellow(), $message);
    };
}

pub(crate) use error;
pub(crate) use success;
pub(crate) use warning;
