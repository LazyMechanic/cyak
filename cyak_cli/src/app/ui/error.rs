use cursive::align::HAlign;
use cursive::view::{Nameable, Scrollable};
use cursive::views::{Dialog, TextView};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Fatal(String),
    #[error("{0}")]
    Regular(String),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }

    pub fn anyhow_fail<T>(self) -> anyhow::Result<T> {
        Err(anyhow::anyhow!(self))
    }
}

pub struct ErrorView;

impl ErrorView {
    pub fn make(err: &Error) -> impl cursive::View {
        let text_view = TextView::new(format!("{}", err)).scrollable();
        match err {
            Error::Fatal(_) => Dialog::around(text_view)
                .title("Fatal error")
                .button("Exit", |s| s.quit())
                .h_align(HAlign::Center)
                .with_name("error"),
            Error::Regular(_) => Dialog::around(text_view)
                .title("Error")
                .dismiss_button("Ok")
                .h_align(HAlign::Center)
                .with_name("error"),
        }
    }
}
