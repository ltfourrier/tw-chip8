use std::fmt;

pub type DWord = u16;
pub type Word = u8;
pub type Nibble = u8;

#[derive(Clone,Copy)]
pub enum Value {
    Register(Nibble),
    Byte(Word),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Register(reg) => write!(f, "V{:X}", reg),
            Value::Byte(b) => write!(f, "{}", b),
        }
    }
}

pub enum Instruction {
    SYS(DWord),
    CLS,
    RET,
    JP(DWord),
    CALL(DWord),
    SE(Nibble, Value),
    SNE(Nibble, Value),
    LD(Nibble, Value),
    ADD(Nibble, Value),
    OR(Nibble, Nibble),
    AND(Nibble, Nibble),
    XOR(Nibble, Nibble),
    SUB(Nibble, Nibble),
    SHR(Nibble, Nibble),
    SUBN(Nibble, Nibble),
    SHL(Nibble, Nibble),
    LDI(DWord),
    JPO(DWord),
    RND(Nibble, Word),
    DRW(Nibble, Nibble, Nibble),
    SKP(Nibble),
    SKNP(Nibble),
    LDDT(Nibble),
    LDK(Nibble),
    LDSDT(Nibble),
    LDSST(Nibble),
    ADDI(Nibble),
    LDF(Nibble),
    LDB(Nibble),
    LDSBLK(Nibble),
    LDBLK(Nibble),
}

fn get_nibble(i: u16, offset: u16) -> u8 {
    (i >> (16 - offset - 4) & 0x0F) as u8
}

fn get_word(i: u16, offset: u16) -> u8 {
    (i >> (16 - offset - 8) & 0xFF) as u8
}

impl Instruction {
    pub fn from_binary(i: u16) -> Result<Instruction, &'static str> {
        use self::Instruction::*;
        match i {
            0x00EE => Ok(RET),
            0x00E0 => Ok(CLS),
            _ if i & 0xF000 == 0x0000 => Ok(SYS(i & 0xFFF)),
            _ if i & 0xF000 == 0x1000 => Ok(JP(i & 0xFFF)),
            _ if i & 0xF000 == 0x2000 => Ok(CALL(i & 0xFFF)),
            _ if i & 0xF000 == 0x3000 => Ok(SE(get_nibble(i, 4), Value::Byte(get_word(i, 8)))),
            _ if i & 0xF000 == 0x4000 => Ok(SNE(get_nibble(i, 4), Value::Byte(get_word(i, 8)))),
            _ if i & 0xF00F == 0x5000 => {
                Ok(SE(get_nibble(i, 4), Value::Register(get_nibble(i, 8))))
            }
            _ if i & 0xF000 == 0x6000 => Ok(LD(get_nibble(i, 4), Value::Byte(get_word(i, 8)))),
            _ if i & 0xF000 == 0x7000 => Ok(ADD(get_nibble(i, 4), Value::Byte(get_word(i, 8)))),
            _ if i & 0xF00F == 0x8000 => {
                Ok(LD(get_nibble(i, 4), Value::Register(get_nibble(i, 8))))
            }
            _ if i & 0xF00F == 0x8001 => Ok(OR(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x8002 => Ok(AND(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x8003 => Ok(XOR(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x8004 => {
                Ok(ADD(get_nibble(i, 4), Value::Register(get_nibble(i, 8))))
            }
            _ if i & 0xF00F == 0x8005 => Ok(SUB(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x8006 => Ok(SHR(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x8007 => Ok(SUBN(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x800E => Ok(SHL(get_nibble(i, 4), get_nibble(i, 8))),
            _ if i & 0xF00F == 0x9000 => {
                Ok(SNE(get_nibble(i, 4), Value::Register(get_nibble(i, 8))))
            }
            _ if i & 0xF000 == 0xA000 => Ok(LDI(i & 0xFFF)),
            _ if i & 0xF000 == 0xB000 => Ok(JPO(i & 0xFFF)),
            _ if i & 0xF000 == 0xC000 => Ok(RND(get_nibble(i, 4), get_word(i, 8))),
            _ if i & 0xF000 == 0xD000 => {
                Ok(DRW(get_nibble(i, 4), get_nibble(i, 8), get_nibble(i, 12)))
            }
            _ if i & 0xF0FF == 0xE09E => Ok(SKP(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xE0A1 => Ok(SKNP(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF007 => Ok(LDDT(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF00A => Ok(LDK(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF015 => Ok(LDSDT(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF018 => Ok(LDSST(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF01E => Ok(ADDI(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF029 => Ok(LDF(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF033 => Ok(LDB(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF055 => Ok(LDSBLK(get_nibble(i, 4))),
            _ if i & 0xF0FF == 0xF065 => Ok(LDBLK(get_nibble(i, 4))),
            _ => Err("instruction does not exist"),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Instruction::*;
        match *self {
            SYS(ref addr) => write!(f, "SYS {:#X}", addr),
            CLS => write!(f, "CLS"),
            RET => write!(f, "RET"),
            JP(ref addr) => write!(f, "JP {:#X}", addr),
            CALL(ref addr) => write!(f, "CALL {:#X}", addr),
            SE(ref reg, ref v) => write!(f, "SE V{}, {}", reg, v),
            SNE(ref reg, ref v) => write!(f, "SNE V{}, {}", reg, v),
            LD(ref reg, ref v) => write!(f, "LD V{}, {}", reg, v),
            ADD(ref reg, ref v) => write!(f, "ADD V{}, {}", reg, v),
            OR(ref reg1, ref reg2) => write!(f, "OR V{}, V{}", reg1, reg2),
            AND(ref reg1, ref reg2) => write!(f, "AND V{}, V{}", reg1, reg2),
            XOR(ref reg1, ref reg2) => write!(f, "XOR V{}, V{}", reg1, reg2),
            SUB(ref reg1, ref reg2) => write!(f, "SUB V{}, V{}", reg1, reg2),
            SHR(ref reg, _) => write!(f, "SHR V{}", reg),
            SUBN(ref reg1, ref reg2) => write!(f, "SUBN V{}, V{}", reg1, reg2),
            SHL(ref reg, _) => write!(f, "SHL V{}", reg),
            LDI(ref addr) => write!(f, "LD I, {:#X}", addr),
            JPO(ref addr) => write!(f, "JP V0, {:#X}", addr),
            RND(ref reg, ref b) => write!(f, "RND V{}, {}", reg, b),
            DRW(ref reg1, ref reg2, ref n) => write!(f, "DRW V{}, V{}, {}", reg1, reg2, n),
            SKP(ref reg) => write!(f, "SKP V{}", reg),
            SKNP(ref reg) => write!(f, "SKNP V{}", reg),
            LDDT(ref reg) => write!(f, "LD V{}, DT", reg),
            LDK(ref reg) => write!(f, "LD V{}, K", reg),
            LDSDT(ref reg) => write!(f, "LD DT, V{}", reg),
            LDSST(ref reg) => write!(f, "LD ST, V{}", reg),
            ADDI(ref reg) => write!(f, "ADD I, V{}", reg),
            LDF(ref reg) => write!(f, "LD F, V{}", reg),
            LDB(ref reg) => write!(f, "LD B, V{}", reg),
            LDSBLK(ref reg) => write!(f, "LD [I], V{}", reg),
            LDBLK(ref reg) => write!(f, "LD V{}, [I]", reg),
        }
    }
}