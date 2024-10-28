
const STACK_SIZE: usize = 32;

pub struct Stack {
    data: [u16; STACK_SIZE],
    ptr: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            ptr: 0
        }
    }

    pub fn push(&mut self, addr: u16) {
        self.data[self.ptr] = addr;
        self.ptr += 1;
    }

    pub fn pop(&mut self) -> Option<u16> {
        if self.ptr == 0 {
            return None;
        }

        self.ptr -= 1;
        Some(self.data[self.ptr])
    }
}