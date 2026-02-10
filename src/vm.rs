use std::process;

use crate::arena::Arena;
use crate::counter::PC;
use crate::registers::Registers;
use crate::stack::Stack;

pub struct VM {
    pub pc: PC,
    pub arena: Arena,
    pub registers: Registers,
    pub stack: Stack,
    pub inst: Option<Instruction>,
}

pub struct Signal {}

pub struct Instruction {
    addr: usize,
    code: u16,
    name: String,
    args: Option<bool>,
}
impl Instruction {
    fn new(name: String) -> Self {
        Self {
            addr: 0,
            code: 0,
            name: name,
            args: None,
        }
    }
}
impl VM {
    pub fn new(arena: Arena) -> Self {
        Self {
            pc: PC::new(),
            arena: arena,
            registers: Registers::new(),
            stack: Stack::new(),
            inst: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.fetch_decode();
            self.execute();
        }
    }

    pub fn fetch_decode(&mut self) {
        let code = self.arena.read(self.pc.get()).unwrap();
        //println!("fetching {:?}", code);
        self.pc.inc();
        match code {
            00 => {
                let inst = Instruction::new("halt".into());
                self.inst = Some(inst);
                process::exit(0);
            }
            01 => {
                let arg1 = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();
                let arg2 = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                self.registers.set(arg1 as usize, arg2);
            }
            02 => {
                let arg1 = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                self.stack.push([arg1]);
            }
            03 => {
                let arg1 = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let poped = self.stack.pop().unwrap()[0];
                if poped >= 32768 {
                    // write to register
                    self.registers.set(arg1 as usize - 32768, poped);
                } else {
                    self.arena.write(arg1 as usize, poped);
                }
            }
            04 => {
                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if b == c {
                    if a >= 32768 {
                        // write to register
                        self.registers.set(a as usize - 32768, 1);
                    } else {
                        self.arena.write(a as usize, 1);
                    }
                } else {
                    if a >= 32768 {
                        // write to register
                        self.registers.set(a as usize - 32768, 0);
                    } else {
                        self.arena.write(a as usize, 0);
                    }
                }
            }
            05 => {
                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if b > c {
                    if a >= 32768 {
                        // write to register
                        self.registers.set(a as usize - 32768, 1);
                    } else {
                        self.arena.write(a as usize, 1);
                    }
                } else {
                    if a >= 32768 {
                        // write to register
                        self.registers.set(a as usize - 32768, 0);
                    } else {
                        self.arena.write(a as usize, 0);
                    }
                }
            }
            06 => {
                let arg1 = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();
                self.pc.jump(arg1 as usize);
            }
            07 => {
                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();
                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a != 0 {
                    self.pc.jump(b as usize);
                }
            }
            08 => {
                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();
                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a == 0 {
                    self.pc.jump(b as usize);
                }
            }
            09 => {
                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a >= 32768 {
                    // write to register
                    self.registers.set(a as usize - 32768, a + b);
                } else {
                    self.arena.write(a as usize, a + b);
                }
            }
            10 => {

                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a >= 32768 {
                    // write to register
                    self.registers.set(a as usize - 32768, a * b);
                } else {
                    self.arena.write(a as usize, a * b );
                }
            }
            11 => {

                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a >= 32768 {
                    // write to register
                    self.registers.set(a as usize - 32768, (a - b) % 32768 );
                } else {
                    self.arena.write(a as usize, a - b  );
                }
            }
            12 => {

                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a >= 32768 {
                    // write to register
                    self.registers.set(a as usize - 32768, a & b );
                } else {
                    self.arena.write(a as usize, a ^ b  );
                }
            }
            13 => {

                let a = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let b = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                let c = self.arena.read(self.pc.get()).unwrap();
                self.pc.inc();

                if a >= 32768 {
                    // write to register
                    self.registers.set(a as usize - 32768, a | b );
                } else {
                    self.arena.write(a as usize, a | b  );
                }
            }
            14 => {}
            15 => {}
            16 => {}
            17 => {}
            18 => {}
            19 => {
                let arg = self.arena.read(self.pc.get()).unwrap();
                let char = u16_to_ascii(arg).unwrap();
                self.pc.inc();
                print!("{}", char);
            }
            20 => {}
            21 => {}
            22 => (),
            _ => (),
        }
    }

    pub fn execute(&mut self) -> Option<Signal> {
        if self.inst.is_some() {}
        None
    }
}

fn u16_to_ascii(code: u16) -> Option<char> {
    if code <= 0x7F {
        // valid ASCII range
        Some(code as u8 as char)
    } else {
        None // not a valid ASCII character
    }
}
