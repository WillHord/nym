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
            helpers::messages::print_error(style(s), false)
        } else if let Some(styled) = (val as &dyn Any).downcast_ref::<StyledObject<&str>>() {
            helpers::messages::print_error(styled.clone(), false)
        } else {
            panic!("Error requires either a &str or a StyledObject");
        }
    }};
    ($message: expr, $exit: expr) => {{
        use console::{style, StyledObject};
        use std::any::Any;
        fn print_type_of<T>(_: &T) {
            println!("{}", std::any::type_name::<T>())
        }

        // print_type_of($message);
        let val = &$message;
        if let Some(s) = (val as &dyn Any).downcast_ref::<&str>() {
            helpers::messages::print_error(style(s), $exit)
        } else if let Some(styled) = (val as &dyn Any).downcast_ref::<StyledObject<&str>>() {
            helpers::messages::print_error(styled.clone(), $exit)
        } else {
            // helpers::messages::print_error($message, $exit)
            // print_type_of($message);
            // panic!("Error requires either a &str or a StyledObject");
        }
    }};
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
