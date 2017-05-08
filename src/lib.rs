extern crate sdl2;

mod cpu;
mod memory;
mod ui;

use std::io;
use std::error::Error;

pub fn run<T>(data: Vec<u8>, dump_file: &mut Option<T>) -> Result<(), Box<Error>>
    where T: io::Write
{
    // Create the UI context (renderer, sound, window...)
    let mut ui = ui::UiContext::new("TW-Chip8")?;

    // Now create the CPU and load the ROM into memory
    let mut cpu = cpu::CPU::new();
    cpu.load_rom(data);

    let mut running = true;
    while running {
        ui.update();
        cpu.step()?;
        if ui.events.quit || !cpu.is_running() {
            running = false;
        }
    }

    if let Some(ref mut f) = *dump_file {
        cpu.dump_memory(f)?;
    }
    Ok(())
}

pub fn disassemble(data: Vec<u8>) {
    let iter = data.chunks(2);
    let mut addr = 0x200;
    for bytes in iter {
        let dword = (bytes[0] as u16 & 0xFF) << 8 | bytes[1] as u16 & 0xFF;
        match cpu::inst::Instruction::from_binary(dword) {
            Ok(inst) => println!("{:#X}\t| {}", addr, inst),
            Err(_) => println!("{:#X}\t| NOP", addr),
        }
        addr += 2;
    }
}