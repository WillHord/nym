use console::style;
use console::StyledObject;

pub enum Message<'a> {
    Styled(StyledObject<String>),
    Plain(&'a str),
}

pub fn success(message: StyledObject<&str>) {
    println!("{}: {}", style("Success").bold().green(), message);
}

pub fn warning(message: StyledObject<&str>) {
    println!("{}: {}", style("Warning").bold().yellow(), message);
}

// pub fn warning_with_action {}

pub fn print_error(message: StyledObject<&str>, exit: bool) {
    eprintln!("{}: {}", style("Error").bold().red(), message);
    if exit {
        std::process::exit(1)
    }
}

#[macro_export]
macro_rules! error {
    ($message: expr) => {
        crate::helpers::messages::print_error($message, false);
    };
    ($message: expr, $exit: expr) => {
        crate::helpers::messages::print_error($message, $exit);
    };
}

pub(crate) use error;
