use crate::prelude::*;

// --- constants --------------------------------------------------------------

/// This represents the size limit of the call stack
const STACK_SIZE: usize = 32;

// --- stack definition -------------------------------------------------------

pub(crate) struct CallStack {
    /// The underlying array holding the data of the stack
    data: [u16; STACK_SIZE],
    /// The stack pointer pointing to the next open slot in `data`
    ptr: usize,
}

impl CallStack {
    /// Creates a new instance of the address stack
    /// 
    /// This will initialize the buffer and the stack pointer to `0`
    /// indicating that the stack is empty
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
            ptr: 0,
        }
    }

    /// Pushes an address onto the stack
    ///
    /// # Params
    ///
    /// - `addr` - The address to push onto the stack
    /// 
    /// # Errors
    /// 
    /// If the stack limit has been reached
    #[inline(always)]
    pub const fn push(&mut self, addr: u16) -> Result<()> {
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
    #[inline(always)]
    pub const fn pop(&mut self) -> Option<u16> {
        if self.ptr == 0 {
            return None;
        }

        self.ptr -= 1;
        Some(self.data[self.ptr])
    }
}
