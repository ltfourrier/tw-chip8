use std::error::Error;
use std::fmt;

mod variant;

const MAX_ARGUMENTS: usize = 3;

#[derive(PartialEq, Debug)]
pub enum InstructionError {
    UnresolvedLabel,
}

impl Error for InstructionError {
    fn description(&self) -> &str {
        match *self {
            InstructionError::UnresolvedLabel => {
                "a label must be resolved before converting to binary"
            }
        }
    }
}

impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unresolved Label")
    }
}

#[derive(PartialEq)]
pub enum Code {
    ClearScreen,
    Return,
    Jump,
    Call,
    SkipEquals,
    SkipNotEquals,
    Load,
    Add,
    Or,
    And,
    Xor,
    Subtract,
    ShiftRight,
    SubtractInverse,
    ShiftLeft,
    Rand,
    Draw,
    SkipKeyPressed,
    SkipKeyNotPressed,
}

#[derive(PartialEq)]
pub enum Argument {
    None,
    Address(u16),
    Register(u8),
    Byte(u8),
    ImagePointer,
    KeyRegister,
    DelayTimer,
    SoundTimer,
    FontPointer,
    BcdPointer,
    ArrayPointer,
    Label(String),
}

impl Argument {
    pub fn get_value(&self) -> Result<u16, InstructionError> {
        match *self {
            Argument::Address(ref a) => Ok(*a),
            Argument::Register(ref r) => Ok(*r as u16),
            Argument::Byte(ref b) => Ok(*b as u16),
            Argument::Label(_) => Err(InstructionError::UnresolvedLabel),
            _ => Ok(0),
        }
    }
}

pub struct Instruction {
    pub code: Code,
    pub arguments: [Argument; MAX_ARGUMENTS],
}

#[cfg(test)]
mod tests {
    use instruction::*;

    #[test]
    fn arg_value_test() {
        let static_arg = Argument::ImagePointer;
        assert_eq!(static_arg.get_value().unwrap(), 0u16);

        let address_arg = Argument::Address(0xDED);
        assert_eq!(address_arg.get_value().unwrap(), 0xDEDu16);

        let register_arg = Argument::Register(10);
        assert_eq!(register_arg.get_value().unwrap(), 10u16);

        let byte_arg = Argument::Byte(142);
        assert_eq!(byte_arg.get_value().unwrap(), 142u16);

        let label_arg = Argument::Label(String::from("hello"));
        assert_eq!(
            label_arg.get_value().unwrap_err(),
            InstructionError::UnresolvedLabel
        );
    }
}
