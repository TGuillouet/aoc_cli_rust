use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct DayNotFoundError;
impl Error for DayNotFoundError {}
impl fmt::Display for DayNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This day could not be found")
    }
}
