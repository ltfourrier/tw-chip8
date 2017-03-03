pub mod inst;
mod error;

use std::io;

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
    running: bool,
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
            running: false,
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

    pub fn run(&mut self) -> Result<(), CPUError> {
        self.running = true;
        loop {
            let inst_dword = try!(self.memory
                .read_dword(self.pc as usize)
                .map_err(|err| CPUError::MemoryError(err)));
            if let Ok(inst) = inst::Instruction::from_binary(inst_dword) {
                println!("{:#X}\t| {}", self.pc, inst);
                try!(self.execute(inst));
            }

            if !self.running {
                break;
            }
        }
        Ok(())
    }

    fn execute(&mut self, inst: inst::Instruction) -> Result<(), CPUError> {
        use self::inst::Instruction::*;
        match inst {
            SYS(addr) => Ok(self.op_sys(addr)),
            RET => self.op_ret(),
            JP(addr) => Ok(self.op_jp(addr)),
            CALL(addr) => self.op_call(addr),
            SE(reg, val) => self.op_se(reg, val),
            SNE(reg, val) => self.op_sne(reg, val),
            LD(reg, val) => self.op_ld(reg, val),
            ADD(reg, val) => self.op_add(reg, val),
            OR(l_reg, r_reg) => self.op_or(l_reg, r_reg),
            AND(l_reg, r_reg) => self.op_and(l_reg, r_reg),
            XOR(l_reg, r_reg) => self.op_xor(l_reg, r_reg),
            SUB(l_reg, r_reg) => self.op_sub(l_reg, r_reg),
            SHR(reg, _) => self.op_shr(reg),
            _ => unimplemented!(),
        }
    }

    fn op_sys(&mut self, addr: inst::DWord) {
        if addr == 0x100 {
            self.running = false;
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

    fn op_se(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let first = try!(self.get_register(reg));
        let second = try!(self.unwrap_value(val));

        self.pc += if first == second { 4 } else { 2 };
        Ok(())
    }

    fn op_sne(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let first = try!(self.get_register(reg));
        let second = try!(self.unwrap_value(val));

        self.pc += if first == second { 2 } else { 4 };
        Ok(())
    }

    fn op_ld(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let val = try!(self.unwrap_value(val));
        try!(self.set_register(reg, val));

        self.pc += 2;
        Ok(())
    }

    fn op_add(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let dst = try!(self.get_register(reg));
        let add = try!(self.unwrap_value(val));

        let dst = dst as u16 + add as u16;
        if let inst::Value::Register(_) = val {
            try!(self.set_register(15, if dst > 255 { 1 } else { 0 }));
        }
        try!(self.set_register(reg, (dst & 0xFF) as u8));

        self.pc += 2;
        Ok(())
    }

    fn op_or(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = try!(self.get_register(l_reg));
        let right = try!(self.get_register(r_reg));
        try!(self.set_register(l_reg, left | right));

        self.pc += 2;
        Ok(())
    }

    fn op_and(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = try!(self.get_register(l_reg));
        let right = try!(self.get_register(r_reg));
        try!(self.set_register(l_reg, left & right));

        self.pc += 2;
        Ok(())
    }

    fn op_xor(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = try!(self.get_register(l_reg));
        let right = try!(self.get_register(r_reg));
        try!(self.set_register(l_reg, left ^ right));

        self.pc += 2;
        Ok(())
    }

    fn op_sub(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = try!(self.get_register(l_reg));
        let right = try!(self.get_register(r_reg));
        try!(self.set_register(15, if left > right { 1 } else { 0 }));
        try!(self.set_register(l_reg, left.wrapping_sub(right)));

        self.pc += 2;
        Ok(())
    }

    fn op_shr(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let reg_val = try!(self.get_register(reg));
        try!(self.set_register(15, if reg_val & 1 == 1 { 1 } else { 0 }));
        try!(self.set_register(reg, reg_val >> 1));

        self.pc += 2;
        Ok(())
    }

    fn unwrap_value(&self, val: inst::Value) -> Result<u8, CPUError> {
        match val {
            inst::Value::Register(reg) => self.get_register(reg),
            inst::Value::Byte(b) => Ok(b),
        }
    }

    fn get_register(&self, reg: inst::Nibble) -> Result<u8, CPUError> {
        let reg_usize = reg as usize;
        if reg_usize < V_REGISTER_COUNT {
            Ok(self.v_registers[reg_usize])
        } else {
            Err(CPUError::InvalidRegister(reg))
        }
    }

    fn set_register(&mut self, reg: inst::Nibble, v: u8) -> Result<(), CPUError> {
        let reg_usize = reg as usize;
        if reg_usize < V_REGISTER_COUNT {
            self.v_registers[reg_usize] = v;
            Ok(())
        } else {
            Err(CPUError::InvalidRegister(reg))
        }
    }
}