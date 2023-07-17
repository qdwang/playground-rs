use std::error::Error;
pub(crate) struct Report {
    location: core::panic::Location<'static>,
    err: Box<dyn Error>,
}
pub(crate) trait ToReport<T> {
    fn to_report(self) -> Result<T, Report>;
}
impl<T, E: std::error::Error + 'static> ToReport<T> for Result<T, E> {
    #[track_caller]
    fn to_report(self) -> Result<T, Report> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => Err(Report {
                location: *core::panic::Location::caller(),
                err: e.into(),
            }),
        }
    }
}
impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} -> {:?}",
            self.location.file(),
            self.location.line(),
            self.err
        )
    }
}
impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for Report {
    /// This method will ignore the report stack and get the first real source in ErrLog
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.err.downcast_ref::<Report>() {
            Some(e) => e.source(),
            None => Some(self.err.as_ref()),
        }
    }
}
