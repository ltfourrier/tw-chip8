use std::fmt;
use std::error::Error;

use super::super::memory;

#[derive(Debug)]
pub enum CPUError {
    StackUnderflow,
    StackOverflow,
    InvalidRegister(u8),
    MemoryError(memory::MemoryError),
    ParsingError(&'static str),
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CPUError::StackUnderflow => write!(f, "attempting to return from an empty stack"),
            CPUError::StackOverflow => write!(f, "stack overflow"),
            CPUError::InvalidRegister(reg) => write!(f, "register {} is invalid", reg),
            CPUError::MemoryError(ref err) => write!(f, "memory error: {}", err),
            CPUError::ParsingError(ref reason) => write!(f, "parsing error: {}", reason),
        }
    }
}

impl Error for CPUError {
    fn description(&self) -> &str {
        match *self {
            CPUError::StackUnderflow => "stack underflow",
            CPUError::StackOverflow => "stack overflow",
            CPUError::InvalidRegister(_) => "invalid register",
            CPUError::MemoryError(ref err) => err.description(),
            CPUError::ParsingError(_) => "parsing error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CPUError::MemoryError(ref err) => Some(err),
            _ => None,
        }
    }
}