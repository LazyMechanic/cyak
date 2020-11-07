pub trait ResultExt<T, E: std::error::Error> {
    fn or_log_err(self) -> Result<T, E>;
}

impl<T, E: std::error::Error> ResultExt<T, E> for Result<T, E> {
    fn or_log_err(self) -> Result<T, E> {
        self.or_else(|err| {
            log::error!("{:?}", err);
            Err(err)
        })
    }
}
