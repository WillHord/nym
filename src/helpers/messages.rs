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
    ($message: expr) => {{
        use console::{style, StyledObject};
        use std::any::Any;

        let val = &$message;
        if let Some(s) = (val as &dyn Any).downcast_ref::<&str>() {
            println!("String: {}", s);
            helpers::messages::print_error(style(s), false)
        } else if let Some(styled) = (val as &dyn Any).downcast_ref::<StyledObject<&str>>() {
            println!("StyledObject: {}", styled);
            helpers::messages::print_error(styled, false)
        }
    }};
    ($message: expr, $exit: expr) => {
        helpers::messages::print_error($message, $exit);
    };
}
// #[macro_export]
// macro_rules! error {
//     ($message: expr) => {
//         helpers::messages::print_error($message, false);
//     };
//     ($message: expr, $exit: expr) => {
//         helpers::messages::print_error($message, $exit);
//     };
// }

#[macro_export]
macro_rules! success {
    ($message: expr) => {
        helpers::messages::success($message)
    };
}

pub(crate) use error;
