pub mod inst;
mod error;

pub fn parse_binary(i: u16) -> Result<Instruction, > {
    use self::inst::Instruction::*;
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