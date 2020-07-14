#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Overflow of text limit: limit = {1}, text size = {}", .1.len())]
    TextLimitOverflow(usize, String),
}

impl Error {
    pub fn fail<T>(self) -> std::result::Result<T, Self> {
        std::result::Result::Err(self)
    }
}
