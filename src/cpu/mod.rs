pub mod inst;

use std::io;

use super::memory;

const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

pub struct CPU {
    v_registers: [u8; V_REGISTER_COUNT],
    i_register: u16,
    pc: u16,
    sp: u8,
    stack: [u8; STACK_SIZE],
    memory: memory::Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            v_registers: [0u8; V_REGISTER_COUNT],
            i_register: 0u16,
            pc: 0x200u16,
            sp: 0u8,
            stack: [0u8; STACK_SIZE],
            memory: memory::Memory::new(),
        }
    }

    pub fn dump_memory<T>(&self, out: &mut T) -> io::Result<usize> where T: io::Write {
        self.memory.dump(out)
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(rom);
    }
}