pub mod inst;
mod error;

use std::io;
use std::error::Error;

use super::memory;
pub use self::error::CPUError;

const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

pub struct CPU {
    v_registers: [u8; V_REGISTER_COUNT],
    i_register: u16,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    memory: memory::Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            v_registers: [0u8; V_REGISTER_COUNT],
            i_register: 0u16,
            pc: 0x200u16,
            sp: 0u8,
            stack: [0u16; STACK_SIZE],
            memory: memory::Memory::new(),
        }
    }

    pub fn dump_memory<T>(&self, out: &mut T) -> io::Result<usize>
        where T: io::Write
    {
        self.memory.dump(out)
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }

    pub fn run(&mut self) -> Result<(), Box<Error>> {
        loop {
            let inst = inst::Instruction::from_binary(self.memory.read_dword(self.pc as usize)?);
            if let Ok(i) = inst {
                self.execute(i)?;
            }

            println!("{:#X}", self.pc);
            if self.pc == 0x100 {
                break;
            }
        }
        Ok(())
    }

    fn execute(&mut self, inst: inst::Instruction) -> Result<(), Box<Error>> {
        use self::inst::Instruction::*;
        match inst {
            SYS(addr) => self.op_sys(addr),
            RET => self.op_ret()?,
            JP(addr) => self.op_jp(addr),
            CALL(addr) => self.op_call(addr)?,
            _ => unimplemented!(),
        }
        Ok(())
    }

    fn op_sys(&mut self, addr: inst::DWord) {
        if addr == 0x100 {
            self.pc = 0x100;
        } else {
            self.pc += 2;
        }
    }

    fn op_ret(&mut self) -> Result<(), CPUError> {
        if self.sp > 0 {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
            Ok(())
        } else {
            Err(CPUError::StackUnderflow)
        }
    }

    fn op_jp(&mut self, addr: inst::DWord) {
        self.pc = addr;
    }

    fn op_call(&mut self, addr: inst::DWord) -> Result<(), CPUError> {
        let sp_usize = self.sp as usize;
        if sp_usize < STACK_SIZE {
            self.stack[sp_usize] = self.pc + 2;
            self.sp += 1;
            self.pc = addr;
            Ok(())
        } else {
            Err(CPUError::StackOverflow)
        }
    }
}