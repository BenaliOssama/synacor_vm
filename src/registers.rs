use crate::registers;

pub struct Registers {
    registers: [u16; 8], // 8 registers
}

impl Registers {
    pub fn new() -> Self {
        Self {
            registers: [0; 8], // initialize all 8 elements to 0
        }
    }

    pub fn set(&mut self, reg: usize, val: u16){
        self.registers[reg] = val;
    }
}
