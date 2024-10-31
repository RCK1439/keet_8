//! This module, `emulator`, is the actual implementation of the Chip-8
//! emulator.
//! 
//! This includes things like the memory, call stack, opcode, instruction
//! and address mode implementations aswell
//! 
//! This module only exposes one other submodule, being the `opcode` module for
//! errors. This also exposes the `Emulator` struct for the application to
//! interact with during runtime.

mod memory;
pub mod opcode;
mod stack;

use memory::Memory;
use opcode::{AddressMode, OpCode};
use stack::CallStack;

use crate::prelude::*;

use raylib::prelude::*;

// --- constants --------------------------------------------------------------

/// Represents the number of available registers to Chip-8
const NUM_REGISTERS: usize = 16;
/// Represents the number keys on the keypad available to Chip-8
const NUM_KEYS: usize = 16;

/// Represents the width of the screen buffer
const VIDEO_BUFFER_WIDTH: usize = 64;
/// Represents the height of the screen buffer
const VIDEO_BUFFER_HEIGHT: usize = 32;

/// Represents the color of a single pixel on the screen buffer
/// 
/// This is the `GREEN` macro used by raylib in C (it is different for some
/// reason here in Rust)
const PIXEL_COLOR: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
};
/// Represents the scaling factor at which pixels are drawn to the window
const SCALE: i32 = crate::WINDOW_WIDTH / VIDEO_BUFFER_WIDTH as i32;

// --- type definitions -------------------------------------------------------

type Executor = fn(&mut Emulator, opcode: OpCode) -> Result<()>;

// --- emulator definition ----------------------------------------------------

pub(crate) struct Emulator {
    /// These are the `V` registers
    registers: [u8; NUM_REGISTERS],

    /// This is the index register
    idx: u16,
    /// This is the program counter
    program_counter: u16,

    /// This is the delay timer
    delay_timer: u8,
    /// This is the sound timer
    sound_timer: u8,

    /// This is the call stack
    stack: CallStack,
    /// This is the available memory to Chip-8
    memory: Memory,
    /// This is the screen buffer
    video_buffer: [u8; VIDEO_BUFFER_WIDTH * VIDEO_BUFFER_HEIGHT],
    /// This is a small array containing the state of the keys
    keypad: [u8; NUM_KEYS],

    /// These are all the executor functions available to our Chip-8
    /// implementation
    instructions: [Executor; 21],
}

impl Emulator {
    /// Creates a new instance of the Chip-8 emulator and initializes all the
    /// systems required for emulation.
    ///
    /// # Params
    ///
    /// - `rom_file` - The filepath to the ROM file
    ///
    /// # Errors
    ///
    /// If there was an error when loading the ROM file
    pub fn new(rom_file: &str) -> Result<Self> {
        Ok(Self {
            registers: [0; NUM_REGISTERS],

            idx: 0,
            program_counter: memory::PROG_ADDR,

            delay_timer: 0,
            sound_timer: 0,

            stack: CallStack::new(),
            memory: Memory::new(rom_file)?,
            video_buffer: [0; VIDEO_BUFFER_WIDTH * VIDEO_BUFFER_HEIGHT],
            keypad: [0; NUM_KEYS],

            instructions: [
                Self::raw,
                Self::cls,
                Self::ret,
                Self::sys,
                Self::jp,
                Self::call,
                Self::se,
                Self::sne,
                Self::ld,
                Self::add,
                Self::or,
                Self::and,
                Self::xor,
                Self::sub,
                Self::shr,
                Self::subn,
                Self::shl,
                Self::rnd,
                Self::drw,
                Self::skp,
                Self::sknp,
            ],
        })
    }

    /// Emulates one CPU cycle by stepping one single instruction
    ///
    /// # Errors
    ///
    /// If an invalid address mode was encountered
    pub fn step(&mut self) -> Result<()> {
        let raw = ((self.memory[self.program_counter] as u16) << 8)
            | (self.memory[self.program_counter + 1] as u16);
        self.program_counter += 2;

        let opcode = OpCode::from(raw);
        self.instructions[opcode.instr as usize](self, opcode)?;

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        Ok(())
    }

    /// Assigns a value to the key
    ///
    /// # Params
    ///
    /// - `key` - The key to assign the value to
    /// - `val` - The value to assign to the key
    pub fn set_key(&mut self, key: usize, val: u8) {
        self.keypad[key] = val;
    }

    /// Draws the video buffer data to the window
    ///
    /// # Params
    ///
    /// - `d` - The draw handle provided by raylib
    pub fn draw_buffer(&mut self, d: &mut RaylibDrawHandle) {
        for y in 0..VIDEO_BUFFER_HEIGHT {
            for x in 0..VIDEO_BUFFER_WIDTH {
                if self.video_buffer[x + y * VIDEO_BUFFER_WIDTH] > 0 {
                    d.draw_rectangle(
                        x as i32 * SCALE,
                        y as i32 * SCALE,
                        SCALE,
                        SCALE,
                        PIXEL_COLOR,
                    );
                }
            }
        }
    }

    /// Executes the `RAW` instruction.
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing execution context
    ///
    /// # Errors
    ///
    /// This function doesn't error, but has to return a result due to the
    /// definition of [Executor]
    fn raw(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        Ok(())
    }

    /// Executes the `CLS` instruction
    ///
    /// This clears the video buffer, essentially clearing the screen
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// This function doesn't error, but has to return a result due to the
    /// definition of [Executor]
    fn cls(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        self.video_buffer.fill(0x00);
        Ok(())
    }

    /// Executes the `RET` instruction
    ///
    /// This returns from the current function the program counter is in
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If the call stack was empty when attempting to pop the previous
    /// address off of
    fn ret(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        if let Some(addr) = self.stack.pop() {
            self.program_counter = addr;
        } else {
            return Err(Keet8Error::CallStackEmpty);
        }

        Ok(())
    }

    /// Executes the `SYS` instruction
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// This function doesn't error, but has to return a result due to the
    /// definition of `Executor`
    fn sys(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        Ok(())
    }

    /// Executes the `JP` instruction
    ///
    /// This sets the program counter to an address to jump to
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn jp(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::Addr { address } => {
                self.program_counter = address;
            }
            AddressMode::V0Addr { address } => {
                self.program_counter = self.registers[0x00] as u16 + address
            }
            _ => return Err(Keet8Error::InvalidAddressMode(opcode.address_mode)),
        }

        Ok(())
    }

    /// Executes the `CALL` instruction
    ///
    /// This does a function call by means of pushing the current value of the
    /// program counter and then jumping to the address of the called function
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// - If an invalid address mode was provided
    /// - If the call stack limit has been reached
    fn call(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::Addr { address } = opcode.address_mode {
            self.stack.push(self.program_counter)?;
            self.program_counter = address;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SE` instruction
    ///
    /// Skips the instruction if two values are equal
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn se(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                if self.registers[x] == byte {
                    self.program_counter += 2;
                }
            }
            AddressMode::VxVy { x, y } => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }
            _ => return Err(Keet8Error::InvalidAddressMode(opcode.address_mode)),
        }

        Ok(())
    }

    /// Executes the `SNE` instruction
    ///
    /// Skips the instruction if two values are not equal
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn sne(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                if self.registers[x] != byte {
                    self.program_counter += 2;
                }
            }
            AddressMode::VxVy { x, y } => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }
            _ => return Err(Keet8Error::InvalidAddressMode(opcode.address_mode)),
        }

        Ok(())
    }

    /// Executes the `LD` instruction
    ///
    /// Loads a value into the specified register
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn ld(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                self.registers[x] = byte;
            }
            AddressMode::VxVy { x, y } => {
                self.registers[x] = self.registers[y];
            }
            AddressMode::IAddr { address } => {
                self.idx = address;
            }
            AddressMode::VxDt { x } => {
                self.registers[x] = self.delay_timer;
            }
            AddressMode::VxKey { x } => {
                let mut found = false;
                for i in 0..NUM_KEYS {
                    if self.keypad[i] > 0 {
                        found = true;
                        self.registers[x] = i as u8;
                        break;
                    }
                }

                if !found {
                    self.program_counter -= 2;
                }
            }
            AddressMode::DtVx { x } => {
                self.delay_timer = self.registers[x];
            }
            AddressMode::StVx { x } => {
                self.sound_timer = self.registers[x];
            }
            AddressMode::FontVx { x } => {
                let digit = self.registers[x];
                self.idx = memory::FONT_ADDR + (5 * digit as u16);
            }
            AddressMode::BcdVx { x } => {
                let mut value = self.registers[x];
                self.memory[self.idx + 2] = value % 10;

                value /= 10;
                self.memory[self.idx + 1] = value % 10;

                value /= 10;
                self.memory[self.idx + 0] = value % 10;
            }
            AddressMode::AddrIVx { x } => {
                (0..=x).for_each(|i| self.memory[self.idx + i as u16] = self.registers[i]);
            }
            AddressMode::VxAddrI { x } => {
                (0..=x).for_each(|i| self.registers[i] = self.memory[self.idx + i as u16]);
            }
            _ => return Err(Keet8Error::InvalidAddressMode(opcode.address_mode)),
        }

        Ok(())
    }

    /// Executes the `ADD` instruction
    ///
    /// Adds a value to the specified register and sets the overflow flag
    /// if an overflow has occured
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn add(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                self.registers[x] = self.registers[x].overflowing_add(byte).0;
            }
            AddressMode::VxVy { x, y } => {
                let sum = self.registers[x] as u16 + self.registers[y] as u16;

                self.registers[0x0F] = (sum > 0x00FF) as u8;
                self.registers[x] = (sum & 0x00FF) as u8;
            }
            AddressMode::IVx { x } => {
                self.idx += self.registers[x] as u16;
            }
            _ => return Err(Keet8Error::InvalidAddressMode(opcode.address_mode)),
        }

        Ok(())
    }

    /// Executes the `OR` instruction
    ///
    /// Performs a bitwise "or" on with the specified registers
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn or(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y } = opcode.address_mode {
            self.registers[x] |= self.registers[y];
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `AND` instruction
    ///
    /// Performs a bitwise "and" on with the specified registers
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn and(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y } = opcode.address_mode {
            self.registers[x] &= self.registers[y];
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `XOR` instruction
    ///
    /// Performs a bitwise "xor" on with the specified registers
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn xor(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y } = opcode.address_mode {
            self.registers[x] ^= self.registers[y];
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SUB` instruction
    ///
    /// Subtracts the specified registers from one another and sets the
    /// overflow flag if an overflow has occured
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn sub(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y } = opcode.address_mode {
            self.registers[0x0F] = (self.registers[x] > self.registers[y]) as u8;
            self.registers[x] = self.registers[x].overflowing_sub(self.registers[y]).0;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SHR` instruction
    ///
    /// Shifts the value register to the right by one and sets the overflow
    /// flag if an overflow has occured
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn shr(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y: _ } = opcode.address_mode {
            self.registers[0x0F] = self.registers[x] & 0x01;
            self.registers[x] >>= 1;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SUBN` instruction
    ///
    /// Sets the `VX` to the value of `VY` minus `VX` and sets the overflow
    /// flag if an overflow has occured
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn subn(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y } = opcode.address_mode {
            self.registers[0x0F] = (self.registers[y] > self.registers[x]) as u8;
            self.registers[x] = self.registers[y].overflowing_sub(self.registers[x]).0;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SHL` instruction
    ///
    /// Shifts the value register to the left by one and sets the overflow
    /// flag if an overflow has occured
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn shl(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVy { x, y: _ } = opcode.address_mode {
            self.registers[0x0F] = (self.registers[x] & 0x80) >> 7;
            self.registers[x] <<= 1;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `RND` instruction
    ///
    /// Generates a random value between 0 and 255 and masks it by the
    /// specified byte value
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn rnd(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxByte { x, byte } = opcode.address_mode {
            self.registers[x] = rand::random::<u8>() & byte;
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `DRW` instruction
    ///
    /// Display `N`-byte sprite starting at memory location `I` at (`VX`, `VY`)
    /// Each set bit of xored with what's already drawn. `VF` is set to `1` if
    /// a collision occurs. `0` otherwise
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn drw(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::VxVyN { x, y, nibble } = opcode.address_mode {
            let height = nibble;
            let xp = self.registers[x] % VIDEO_BUFFER_WIDTH as u8;
            let yp = self.registers[y] % VIDEO_BUFFER_HEIGHT as u8;

            self.registers[0x0F] = 0;
            for r in 0..height {
                let sprite = self.memory[self.idx + r as u16];
                for c in 0..8 {
                    let sprite_px = sprite & (0x80 >> c);
                    let screen_idx =
                        (yp as usize + r as usize) * VIDEO_BUFFER_WIDTH + (xp as usize + c);

                    if sprite_px > 0 {
                        if self.video_buffer[screen_idx] == 0xFF {
                            self.registers[0x0F] = 1;
                        }

                        self.video_buffer[screen_idx] ^= 0xFF;
                    }
                }
            }
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SKP` instruction
    ///
    /// Skips the next instruction if the specified key is pressed
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn skp(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::Vx { x } = opcode.address_mode {
            let key = self.registers[x];
            if self.keypad[key as usize] > 0 {
                self.program_counter += 2;
            }
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }

    /// Executes the `SKNP` instruction
    ///
    /// Skips the next instruction of the specified key is not pressed
    ///
    /// # Params
    ///
    /// - `opcode` - The opcode containing the execution context
    ///
    /// # Errors
    ///
    /// If an invalid address mode was provided
    fn sknp(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::Vx { x } = opcode.address_mode {
            let key = self.registers[x];
            if self.keypad[key as usize] <= 0 {
                self.program_counter += 2;
            }
        } else {
            return Err(Keet8Error::InvalidAddressMode(opcode.address_mode));
        }

        Ok(())
    }
}
