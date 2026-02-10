use crate::arena::Arena;
use crate::counter::PC;
use crate::registers::Registers;
use crate::stack::Stack;

pub struct VM {
    pub pc: PC,
    pub arena: Arena,
    pub registers: Registers,
    pub stack: Stack,
}

pub struct Signal {

}

impl VM {
    pub fn new(arena: Arena) -> Self {
        Self {
            pc: PC::new(),
            arena: arena,
            registers: Registers::new(),
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            Self::fetch_decode();
            Self::execute();
        }
    }

    pub fn fetch_decode() {

    }

    pub fn execute() -> Option<Signal> {
        None
    }
}
