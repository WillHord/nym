macro_rules! yesno {
    ($message: expr) => {
        Confirm::new(format!("{} {}", $message, "[y/n]").as_str()).prompt()
    };
}

pub(crate) use yesno;
