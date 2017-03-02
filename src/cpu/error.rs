use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CPUError {
    StackUnderflow,
    StackOverflow,
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CPUError::StackUnderflow => write!(f, "attempting to return from an empty stack"),
            CPUError::StackOverflow => write!(f, "stack overflow"),
        }
    }
}

impl Error for CPUError {
    fn description(&self) -> &str {
        match *self {
            CPUError::StackUnderflow => "stack underflow",
            CPUError::StackOverflow => "stack overflow",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}