use std::io;
use std::fmt;
use std::error::Error;

const RAM_SIZE: usize = 0x1000;

static HEX_DIGITS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug)]
pub enum MemoryError {
    ReservedAddress(usize),
    UnmappedAddress(usize),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemoryError::ReservedAddress(addr) => write!(f, "address {} is reserved", addr),
            MemoryError::UnmappedAddress(addr) => write!(f, "address {} is out of bounds", addr),
        }
    }
}

impl Error for MemoryError {
    fn description(&self) -> &str {
        match *self {
            MemoryError::ReservedAddress(_) => "reserved address",
            MemoryError::UnmappedAddress(_) => "address out of bounds",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut ram = [0u8; RAM_SIZE];
        {
            let hex_iter = HEX_DIGITS.iter();
            let iter = ram.iter_mut().skip(0x150).zip(hex_iter);
            for (src, dst) in iter {
                *src = *dst;
            }
        }
        Memory { ram: ram }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let rom_iter = rom.iter().take(RAM_SIZE - 0x200);
        let iter = self.ram.iter_mut().skip(0x200).zip(rom_iter);
        for (src, dst) in iter {
            *src = *dst;
        }
    }

    pub fn dump<T>(&self, out: &mut T) -> io::Result<usize>
        where T: io::Write
    {
        out.write(&self.ram)
    }

    pub fn read_word(&self, addr: usize) -> Result<u8, MemoryError> {
        match addr {
            _ if addr > 0xFFF => Err(MemoryError::UnmappedAddress(addr)),
            _ => Ok(self.ram[addr]),
        }
    }

    pub fn write_word(&mut self, addr: usize, b: u8) -> Result<(), MemoryError> {
        match addr {
            _ if addr < 0x200 => Err(MemoryError::ReservedAddress(addr)),
            _ if addr > 0xFFF => Err(MemoryError::UnmappedAddress(addr)),
            _ => {
                self.ram[addr] = b;
                Ok(())
            }
        }
    }

    pub fn read_dword(&self, addr: usize) -> Result<u16, MemoryError> {
        let w1 = self.read_word(addr)?;
        let w2 = self.read_word(addr + 1)?;
        Ok((w1 as u16 & 0xFF) << 8 | w2 as u16 & 0xFF)
    }
}
