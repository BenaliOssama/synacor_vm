



pub struct Stack(Vec<[u16;1]>);


impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, value: [u16; 1]) {
        self.0.push(value);
    }

    pub fn pop(&mut self) -> Option<[u16; 1]> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&[u16; 1]> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
