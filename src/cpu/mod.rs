extern crate rand;

pub mod inst;
mod error;

use std::io;

use memory;
use com::Communicator;
use com::video::{VideoCommunicator, VideoSignal};
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
            running: true,
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

    pub fn step(&mut self, com: &mut Communicator) -> Result<(), CPUError> {
        let addr = self.pc as usize;
        let inst_dword = self.read_dword(addr)?;
        let inst = inst::Instruction::from_binary(inst_dword)
            .map_err(|reason| CPUError::ParsingError(reason))?;

        //println!("{:#X}\t| {}", self.pc, inst);
        self.execute(inst, com)
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    fn execute(&mut self, inst: inst::Instruction, com: &mut Communicator) -> Result<(), CPUError> {
        use self::inst::Instruction::*;
        match inst {
            SYS(addr) => Ok(self.op_sys(addr)),
            CLS => Ok(self.op_cls(com)),
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
            SUBN(l_reg, r_reg) => self.op_subn(l_reg, r_reg),
            SHL(reg, _) => self.op_shl(reg),
            LDI(addr) => Ok(self.op_ldi(addr)),
            JPO(addr) => Ok(self.op_jpo(addr)),
            RND(reg, mask) => self.op_rnd(reg, mask),
            DRW(x_reg, y_reg, size) => self.op_drw(x_reg, y_reg, size, com),
            ADDI(reg) => self.op_addi(reg),
            LDB(reg) => self.op_ldb(reg),
            LDSBLK(reg) => self.op_ldsblk(reg),
            LDBLK(reg) => self.op_ldblk(reg),
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

    fn op_cls(&mut self, com: &mut Communicator) {
        for pixel in com.video.display.iter_mut() {
            *pixel = false;
        }
        com.video.signal = VideoSignal::Clear;
        self.pc += 2;
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
        let first = self.get_register(reg)?;
        let second = self.unwrap_value(val)?;

        self.pc += if first == second { 4 } else { 2 };
        Ok(())
    }

    fn op_sne(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let first = self.get_register(reg)?;
        let second = self.unwrap_value(val)?;

        self.pc += if first == second { 2 } else { 4 };
        Ok(())
    }

    fn op_ld(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let val = self.unwrap_value(val)?;
        self.set_register(reg, val)?;

        self.pc += 2;
        Ok(())
    }

    fn op_add(&mut self, reg: inst::Nibble, val: inst::Value) -> Result<(), CPUError> {
        let dst = self.get_register(reg)?;
        let add = self.unwrap_value(val)?;

        let dst = dst as u16 + add as u16;
        if let inst::Value::Register(_) = val {
            self.set_register(15, if dst > 255 { 1 } else { 0 })?;
        }
        self.set_register(reg, (dst & 0xFF) as u8)?;

        self.pc += 2;
        Ok(())
    }

    fn op_or(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = self.get_register(l_reg)?;
        let right = self.get_register(r_reg)?;
        self.set_register(l_reg, left | right)?;

        self.pc += 2;
        Ok(())
    }

    fn op_and(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = self.get_register(l_reg)?;
        let right = self.get_register(r_reg)?;
        self.set_register(l_reg, left & right)?;

        self.pc += 2;
        Ok(())
    }

    fn op_xor(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = self.get_register(l_reg)?;
        let right = self.get_register(r_reg)?;
        self.set_register(l_reg, left ^ right)?;

        self.pc += 2;
        Ok(())
    }

    fn op_sub(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = self.get_register(l_reg)?;
        let right = self.get_register(r_reg)?;
        self.set_register(15, if left > right { 1 } else { 0 })?;
        self.set_register(l_reg, left.wrapping_sub(right))?;

        self.pc += 2;
        Ok(())
    }

    fn op_shr(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let reg_val = self.get_register(reg)?;
        self.set_register(15, if reg_val & 1 == 1 { 1 } else { 0 })?;
        self.set_register(reg, reg_val >> 1)?;

        self.pc += 2;
        Ok(())
    }

    fn op_subn(&mut self, l_reg: inst::Nibble, r_reg: inst::Nibble) -> Result<(), CPUError> {
        let left = self.get_register(l_reg)?;
        let right = self.get_register(r_reg)?;
        self.set_register(15, if right > left { 1 } else { 0 })?;
        self.set_register(l_reg, right.wrapping_sub(left))?;

        self.pc += 2;
        Ok(())
    }

    fn op_shl(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let reg_val = self.get_register(reg)?;
        self.set_register(15, if reg_val & 0x80 == 0x80 { 1 } else { 0 })?;
        self.set_register(reg, reg_val << 1)?;

        self.pc += 2;
        Ok(())
    }

    fn op_ldi(&mut self, addr: inst::DWord) {
        self.i_register = addr;
        self.pc += 2;
    }

    fn op_jpo(&mut self, addr: inst::DWord) {
        self.pc = self.v_registers[0] as u16 + addr;
    }

    fn op_rnd(&mut self, reg: inst::Nibble, mask: inst::Word) -> Result<(), CPUError> {
        self.set_register(reg, rand::random::<u8>() & mask)?;

        self.pc += 2;
        Ok(())
    }

    fn op_drw(&mut self,
              x_reg: inst::Nibble,
              y_reg: inst::Nibble,
              size: inst::Nibble,
              com: &mut Communicator)
              -> Result<(), CPUError> {
        let x = self.get_register(x_reg)? as usize;
        let y = self.get_register(y_reg)? as usize;
        let size = size as usize;
        let mut collision = false;

        for line in 0..size {
            let i_register = self.i_register as usize;
            let pixels = self.read_word(i_register + line)?;
            for column in 0..8 {
                let pixel = (pixels & (0x80 >> column)) != 0;
                collision = self.set_pixel(x + column, y + line, pixel, &mut com.video) | collision;
            }
        }

        com.video.signal = VideoSignal::Refresh;
        Ok(())
    }

    fn op_addi(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let reg_val = self.get_register(reg)?;
        self.i_register = self.i_register.wrapping_add(reg_val as u16);

        self.pc += 2;
        Ok(())
    }

    fn op_ldb(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let reg_val = self.get_register(reg)?;
        let addr = self.i_register as usize;
        self.write_word(addr, reg_val / 100)?;
        self.write_word(addr + 1, reg_val % 100 / 10)?;
        self.write_word(addr + 2, reg_val % 10)?;

        self.pc += 2;
        Ok(())
    }

    fn op_ldsblk(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let addr = self.i_register as usize;
        for i in 0..reg {
            let reg_val = self.get_register(i)?;
            self.write_word(addr + i as usize, reg_val)?;
        }

        self.pc += 2;
        Ok(())
    }

    fn op_ldblk(&mut self, reg: inst::Nibble) -> Result<(), CPUError> {
        let addr = self.i_register as usize;
        for i in 0..reg {
            let mem_val = self.read_word(addr + i as usize)?;
            self.set_register(i, mem_val)?;
        }

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

    fn read_word(&mut self, addr: usize) -> Result<u8, CPUError> {
        self.memory
            .read_word(addr)
            .map_err(|err| CPUError::MemoryError(err))
    }

    fn write_word(&mut self, addr: usize, b: u8) -> Result<(), CPUError> {
        self.memory
            .write_word(addr, b)
            .map_err(|err| CPUError::MemoryError(err))
    }

    fn read_dword(&mut self, addr: usize) -> Result<u16, CPUError> {
        self.memory
            .read_dword(addr)
            .map_err(|err| CPUError::MemoryError(err))
    }

    fn set_pixel(&mut self,
                 x: usize,
                 y: usize,
                 pixel: bool,
                 video_com: &mut VideoCommunicator)
                 -> bool {
        // Wrap the pixels around the screen
        let x = x % video_com.width;
        let y = y % video_com.height;

        let idx = video_com.width * y + x;
        let collision = video_com.display[idx] != false && pixel;
        video_com.display[idx] = video_com.display[idx] ^ pixel;
        collision
    }
}