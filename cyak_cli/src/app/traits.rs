pub trait ResultExt<T, E: std::error::Error> {
    fn or_log_err(self) -> Result<T, E>;
}

impl<T, E: std::error::Error> ResultExt<T, E> for Result<T, E> {
    fn or_log_err(self) -> Result<T, E> {
        self.map_err(|err| {
            log::error!("{:?}", err);
            err
        })
    }
}
