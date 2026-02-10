mod arena;
mod counter;
mod stack;
mod registers;
mod vm;

use arena::Arena;
use counter::PC;
use stack::Stack;
use registers::Registers;
use vm::VM;

use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()>{
    let mut file = File::open("challenge.bin")?; 
    let mut arena = Arena::new();


    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    arena.load(&buffer);
    println!("Read {} bytes", buffer.len());
    println!("{}", arena);

    let mut vm = VM::new(arena.clone());

    vm.run();

    Ok(())
}


