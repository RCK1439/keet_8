use std::fmt::Display;

use crate::emulator::opcode::AddressMode;

pub enum Keet8Error {
    NoROMFile,
    FailedToLoadROM(String),

    StackEmpty,
    InvalidAddressMode(AddressMode),
}

impl Display for Keet8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoROMFile => write!(f, "No ROM file specified"),
            Self::FailedToLoadROM(rom) => write!(f, "Failed to load ROM: {rom}"),
            Self::StackEmpty => write!(f, "Stack empty"),
            Self::InvalidAddressMode(addr_mode) => write!(f, "Invalid address mode: {addr_mode}"),
        }
    }
}
