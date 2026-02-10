# Synacor VM

A fast, lightweight **Synacor Virtual Machine** implemented in Rust. Built for a friendly coding competition to see what can be achieved in a single session.

## Overview

The Synacor VM is designed to execute the **Synacor Challenge binary** by implementing the challenge’s architecture:

* **Memory:** 32,768 16-bit words
* **Registers:** 8 general-purpose registers
* **Stack:** dynamic, LIFO
* **Instructions:** all Synacor opcodes
* **I/O:** simple console input/output

This project is a quick, fun implementation to explore the challenge in a hands-on way.

## Features

* Load a Synacor binary into memory from a file
* Read and write memory safely
* Stack-based operations
* Display memory for debugging
* Implemented in **Rust** for safety and speed

## Usage

```bash
cargo run --release -- <path_to_binary>
```

Example in `main.rs`:

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let buffer = fs::read("challenge.bin")?;
    let mut arena = Arena::new();
    arena.load(&buffer);

    println!("{}", arena); // display initial memory state

    Ok(())
}
```

## Project Structure

```
src/
├── main.rs       # entry point
├── arena.rs      # memory, load, read, write
├── registers.rs  # CPU registers
└── stack.rs      # stack implementation
```

## Goals

* Fun, casual competition: see what can be implemented in one sitting
* Explore Rust for low-level systems programming
* Understand the Synacor VM architecture by building it from scratch

## License

MIT License – free to use, modify, and share.
