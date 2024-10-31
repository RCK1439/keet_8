use crate::prelude::*;

use std::ops::{Index, IndexMut};

// --- constants --------------------------------------------------------------

/// Represents the starting address of our actual ROM program
pub(crate) const PROG_ADDR: u16 = 0x0200;
/// Represents the starting address of the font data
pub(crate) const FONT_ADDR: u16 = 0x0050;

/// Represents the maximum available memory to Chip-8
const MEMORY_SIZE: usize = 4 * 1024;
/// Represents the size of the `FONTSET` buffer
const FONTSET_SIZE: usize = 80;

// --- memory definition ------------------------------------------------------

pub(crate) struct Memory {
    /// This is the physical memory space of Chip-8
    space: [u8; MEMORY_SIZE],
}

impl Memory {
    /// Creates and initializes the memory for the emulator
    ///
    /// # Params
    ///
    /// - `rom_file` - The filepath to the ROM to load into memory
    ///
    /// # Errors
    ///
    /// - If there was an error when loading the ROM file
    pub fn new(rom_file: &str) -> Result<Self> {
        let bytes = std::fs::read(rom_file)
            .map_err(|_| Keet8Error::FailedToLoadROM(rom_file.to_string()))?;

        let mut space = [0; MEMORY_SIZE];

        (0..bytes.len()).for_each(|i| space[PROG_ADDR as usize + i] = bytes[i]);
        load_font(&mut space);

        Ok(Self { space })
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    /// Gets the value at the specified 16-bit address
    ///
    /// # Params
    ///
    /// - `addr` - The memory address to read
    fn index(&self, addr: u16) -> &Self::Output {
        let idx = (addr & 0x0FFF) as usize;
        &self.space[idx]
    }
}

impl IndexMut<u16> for Memory {
    /// Gets the value at the specified 16-bit address (mutably)
    ///
    /// # Params
    ///
    /// - `addr` - The memory address to read
    fn index_mut(&mut self, addr: u16) -> &mut Self::Output {
        let idx = (addr & 0x0FFF) as usize;
        &mut self.space[idx]
    }
}

// --- utility functions ------------------------------------------------------

/// Loads the font data of Chip-8 into the given buffer
///
/// # Params
///
/// - `buffer` - The buffer to load the font into
fn load_font(buffer: &mut [u8; MEMORY_SIZE]) {
    const FONTSET: [u8; FONTSET_SIZE] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    (0..FONTSET_SIZE).for_each(|i| buffer[FONT_ADDR as usize + i] = FONTSET[i]);
}
