use crate::prelude::*;

// --- constants --------------------------------------------------------------

const STACK_SIZE: usize = 32;

// --- stack definition -------------------------------------------------------

pub struct CallStack {
    data: [u16; STACK_SIZE],
    ptr: usize,
}

impl CallStack {
    /// Creates a new instance of the address stack
    pub fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            ptr: 0
        }
    }

    /// Pushes an address onto the stack
    /// 
    /// # Params
    /// 
    /// - `addr` - The address to push onto the stack
    pub fn push(&mut self, addr: u16) -> Result<()> {
        if self.ptr == STACK_SIZE {
            return Err(Keet8Error::CallStackFull);
        }

        self.data[self.ptr] = addr;
        self.ptr += 1;

        Ok(())
    }

    /// Pops an address from the stack
    /// 
    /// Returns [Some] if the stack has a value
    /// on the stack. Returns [None] otherwise
    pub fn pop(&mut self) -> Option<u16> {
        if self.ptr == 0 {
            return None;
        }

        self.ptr -= 1;
        Some(self.data[self.ptr])
    }
}
