mod memory;
mod stack;
mod opcode;

use memory::Memory;
use stack::Stack;

use crate::prelude::*;

const NUM_REGISTERS: usize = 16;
const VIDEO_BUFFER_WIDTH: usize = 64;
const VIDEO_BUFFER_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16;

pub(crate) struct Emulator {
    registers: [u8; NUM_REGISTERS],

    idx: u16,
    program_counter: u16,

    delay_timer: u8,
    sound_timer: u8,

    stack: Stack,
    memory: Memory,
    video_buffer: [u8; VIDEO_BUFFER_WIDTH * VIDEO_BUFFER_HEIGHT],
    keypad: [u8; NUM_KEYS],
}

impl Emulator {
    pub fn new(rom_file: &str) -> Result<Self> {
        Ok(Self {
            registers: [0; NUM_REGISTERS],

            idx: 0,
            program_counter: 0,

            delay_timer: 0,
            sound_timer: 0,

            stack: Stack::new(),
            memory: Memory::new(rom_file)?,
            video_buffer: [0; VIDEO_BUFFER_WIDTH * VIDEO_BUFFER_HEIGHT],
            keypad: [0; NUM_KEYS]
        })
    }
}