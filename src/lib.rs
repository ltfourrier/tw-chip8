mod cpu;
mod memory;

use std::fs::File;
use std::io::prelude::*;

pub fn run(data: Vec<u8>) {
    let mut cpu = cpu::CPU::new();
    cpu.load_rom(data);

    // DEBUG: Dump the memory to a file
    let mut f = File::create("MEMORY_DUMP").unwrap();
    cpu.dump_memory(&mut f).unwrap();
    f.flush().unwrap();
}

pub fn disassemble(data: Vec<u8>) {
    let iter = data.chunks(2);
    let mut addr = 0x200;
    for bytes in iter {
        let dword = (((bytes[0] as u16) & 0xFF) << 8) | ((bytes[1] as u16) & 0xFF);
        match cpu::inst::Instruction::from_binary(dword) {
            Ok(inst) => println!("{:#X}\t| {}", addr, inst),
            Err(_) => println!("{:#X}\t| NOP", addr),
        }
        addr += 2;
    }
}