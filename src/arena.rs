use std::fmt;

#[derive(Clone)]
pub struct Arena {
    pub memory: [u16; 32768],
}

impl Arena {
    pub fn new() -> Self {
        Self { memory: [0; 32768] }
    }

    pub fn load(&mut self, buffer: &Vec<u8>) {
        let len = buffer.len() / 2; // number of u16 values
        for i in 0..len.min(self.memory.len()) {
            let index = i * 2;
            self.memory[i] = u16::from_le_bytes([buffer[index], buffer[index + 1]]);
        }
    }

        // Read a u16 at a given address
    pub fn read(&self, addr: usize) -> Option<u16> {
        self.memory.get(addr).copied()
    }

    // Write a u16 at a given address
    pub fn write(&mut self, addr: usize, value: u16) -> bool {
        if let Some(slot) = self.memory.get_mut(addr) {
            *slot = value;
            true
        } else {
            false 
        }
    }
}

// Implement Display for Arena
impl fmt::Display for Arena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print memory in rows of 16 values
        for (i, val) in self.memory.iter().enumerate() {
            write!(f, "{:04X} ", val)?; // print as 4-digit hex
            if (i + 1) % 16 == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
