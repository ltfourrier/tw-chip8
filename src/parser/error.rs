use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParserError {
    UnknownInstruction,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParserError::UnknownInstruction => write!(f, "unknown instruction"),
        }
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        match *self {
            ParserError::UnknownInstruction => "unknown instruction",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}