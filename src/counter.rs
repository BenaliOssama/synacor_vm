

pub struct PC {
    pub pc : usize,
}


impl PC {
    pub fn new() -> Self{
        Self {
            pc: 0,
        }
    }

    pub fn inc(&mut self)  {
        self.pc += 1;
        self.pc %= 32768
    }

    pub fn get(&mut self)-> usize{
        return self.pc
    }

    pub fn jump(&mut self, to : usize){
        self.pc = to;
        self.pc %= 32768;
    }
}
