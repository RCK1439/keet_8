use crate::error::Keet8Error;
use crate::prelude::*;

use std::ops::{ Index, IndexMut };

const MEMORY_SIZE: usize = 4 * 1024;
const PROG_START_ADDR: usize = 0x0200;

const FONTSET_SIZE: usize = 80;
const FONT_ADDR: usize = 0x0050;

pub(crate) struct Memory {
    data: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn new(rom_file: &str) -> Result<Self> {
        let bytes = std::fs::read(rom_file)
            .map_err(|_| Keet8Error::FailedToLoadROM(rom_file.to_string()))?;

        let mut data = [0u8; MEMORY_SIZE];
        for i in 0..bytes.len() {
            data[PROG_START_ADDR + i] = bytes[i];
        }

        todo!()
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, addr: u16) -> &Self::Output {
        let idx = (addr & 0x0FFF) as usize;
        &self.data[idx]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, addr: u16) -> &mut Self::Output {
        let idx = (addr & 0x0FFF) as usize;
        &mut self.data[idx]
    }
}

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
    	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    for i in 0..FONTSET_SIZE {
        buffer[FONT_ADDR + i] = FONTSET[i];
    }
}