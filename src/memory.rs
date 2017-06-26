use std::io;

const RAM_SIZE: usize = 0x1000;
const FONT_SIZE: usize = 80;
const FONT_OFFSET: usize = 0x100;

static FONT: [u8; FONT_SIZE] = [
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

pub struct Memory {
    ram: [u8; RAM_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory { ram: [0u8; RAM_SIZE] };
        mem.load_in_ram(FONT.iter(), FONT_OFFSET);
        mem
    }

    pub fn load_in_ram<'a, I>(&mut self, data_iter: I, offset: usize)
    where
        I: Iterator<Item = &'a u8>,
    {
        let iter = self.ram.iter_mut().skip(offset).zip(data_iter);
        for (dst, src) in iter {
            *dst = *src;
        }
    }

    pub fn dump<W>(&self, out: &mut W) -> io::Result<usize>
    where
        W: io::Write,
    {
        out.write(&self.ram)
    }
}

#[cfg(test)]
mod tests {
    use memory::*;

    static SOME_DATA: [u8; 8] = [25, 25, 2, 42, 128, 64, 100, 99];

    #[test]
    fn loading_test() {
        let mut mem = Memory::new();
        mem.load_in_ram(SOME_DATA.iter(), 0x204);
        mem.load_in_ram(SOME_DATA.iter(), 0x200);
        for i in 0..8 {
            assert!(mem.ram[i + 0x200] == SOME_DATA[i]);
        }
        assert!(mem.ram[0x208] == SOME_DATA[4]);
    }

    #[test]
    fn font_test() {
        let mem = Memory::new();
        for i in 0..FONT_SIZE {
            assert!(mem.ram[i + FONT_OFFSET] == FONT[i]);
        }
    }

    #[test]
    fn dump_test() {
        let mut ram_copy = Vec::new();
        let mem = Memory::new();
        assert!(mem.dump(&mut ram_copy).is_ok());
        assert_eq!(ram_copy.as_slice(), &mem.ram[..]);
    }
}
