use std::fmt;
use std::error;

#[derive(Debug)]
pub enum CustomError {
    GetIfAddrsError(String, i32),
    GetIfNameError(String, i32),
}

impl error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::GetIfAddrsError(cmd, code) => {
                write!(f, "Failed to execute '{}'. Received error code '{}'", cmd, code)
            }
            CustomError::GetIfNameError(cmd, code) => {
                write!(f, "Failed to execute '{}'. Received error code '{}'", cmd, code)
            }
        }
    }
}
