mod cpu;
mod memory;

pub fn disassemble(data: Vec<u8>) {
    let mut iter = data.chunks(2);
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