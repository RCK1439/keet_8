use std::fmt::Display;

use crate::emulator::opcode::AddressMode;

// --- error definition -------------------------------------------------------

pub enum Keet8Error {
    /// The ROM file was not specified in the command-line arguments
    NoROMFile,
    /// The ROM could not be loaded into memory
    /// 
    /// Also contains the filepath to the specified ROM
    FailedToLoadROM(String),

    /// There was an attempt to pop from the call stack, but the stack was empty
    CallStackEmpty,
    /// There was an attempt to push onto the call stack, but the stack was full
    CallStackFull,
    /// An invalid address mode was encounted for a instruction
    InvalidAddressMode(AddressMode),
}

impl Display for Keet8Error {
    /// Writes the error to the output stream
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoROMFile => write!(f, "No ROM file specified"),
            Self::FailedToLoadROM(rom) => write!(f, "Failed to load ROM: {rom}"),
            Self::CallStackEmpty => write!(f, "Call stack is empty"),
            Self::CallStackFull => write!(f, "Call stack limit reached"),
            Self::InvalidAddressMode(addr_mode) => write!(f, "Invalid address mode: {addr_mode}"),
        }
    }
}
