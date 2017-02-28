pub mod inst;

const V_REGISTER_COUNT: usize = 16;
const STACK_SIZE: usize = 16;

pub struct CPU {
    v_registers: [u8; V_REGISTER_COUNT],
    i_register: u16,
    pc: u16,
    sp: u8,
    stack: [u8; STACK_SIZE],
}