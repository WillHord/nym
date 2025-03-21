macro_rules! yesno {
    ($message: expr) => {
        inquire::Confirm::new(format!("{} {}", $message, "[y/n]").as_str()).prompt()
    };
}

macro_rules! get_filepath {
    ($message: expr) => {
        inquire::Text::new(format!("{}: ", $message).as_str())
            .with_autocomplete(crate::helpers::filepath_autocomplete::FilePathCompleter::default())
            .prompt()
    };
}

pub(crate) use get_filepath;
pub(crate) use yesno;
