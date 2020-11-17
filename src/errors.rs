use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone,)]
pub struct MissingDataError;

impl fmt::Display for MissingDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MissingDataError")
    }
}
impl Error for MissingDataError {}