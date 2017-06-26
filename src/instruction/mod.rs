mod variant;

const MAX_ARGUMENTS: usize = 3;

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

pub struct Instruction {
    pub code: Code,
    pub arguments: [Argument; MAX_ARGUMENTS],
}
