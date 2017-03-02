use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CPUError {
    StackUnderflow,
    StackOverflow,
    InvalidRegister(u8),
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CPUError::StackUnderflow => write!(f, "attempting to return from an empty stack"),
            CPUError::StackOverflow => write!(f, "stack overflow"),
            CPUError::InvalidRegister(reg) => write!(f, "register {} is invalid", reg),
        }
    }
}

impl Error for CPUError {
    fn description(&self) -> &str {
        match *self {
            CPUError::StackUnderflow => "stack underflow",
            CPUError::StackOverflow => "stack overflow",
            CPUError::InvalidRegister(_) => "invalid register",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}