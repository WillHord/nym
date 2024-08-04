macro_rules! yesno {
    ($message: expr) => {
        inquire::Confirm::new(format!("{} {}", $message, "[y/n]").as_str()).prompt()
    };
}

pub(crate) use yesno;
