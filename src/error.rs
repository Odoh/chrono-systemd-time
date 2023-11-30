/// Describes an error during the parsing of a timestamp.
#[derive(Debug)]
pub enum Error {
    /// The timestamp is incorrectly formatted.
    Format(String),
    /// The timestamp contains a component that cannot be parsed into a number, or the number overflowed.
    Number(String),
    /// The timestamp contains a component that cannot be parsed into a time unit.
    TimeUnit(String),
    /// The timestamp is invalid in the given timezone.
    Never,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Format(emsg) => write!(f, "invalid timestamp format: {emsg}"),
            Error::Number(emsg) => write!(f, "invalid timestamp number: {emsg}"),
            Error::TimeUnit(unit) => write!(f, "invalid time unit: {unit}"),
            Error::Never => write!(f, "invalid timestamp in the given timezone"),
        }
    }
}
