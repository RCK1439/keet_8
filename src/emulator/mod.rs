mod memory;
mod stack;
mod opcode;

use memory::Memory;
use stack::Stack;
use opcode::{ AddressMode, OpCode };

use crate::prelude::*;
use crate::error::Keet8Error;

use raylib::prelude::*;

const NUM_REGISTERS: usize = 16;
const VIDEO_BUFFER_WIDTH: usize = 64;
const VIDEO_BUFFER_HEIGHT: usize = 32;
const NUM_KEYS: usize = 16;

const SCALE: i32 = 1024 / 64;

type Executor = fn(&mut Emulator, opcode: OpCode) -> Result<()>;

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

    instructions: [Executor; 21]
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
                Self::sknp
            ]
        })
    }

    pub fn step(&mut self) -> Result<()> {
        let raw = ((self.memory[self.program_counter] as u16) << 8) | (self.memory[self.program_counter + 1] as u16);
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

    pub fn set_key(&mut self, key: u8, val: u8) {
        self.keypad[key as usize] = val;
    }

    pub fn draw_buffer(&mut self, d: &mut RaylibDrawHandle) {
        for y in 0..VIDEO_BUFFER_HEIGHT {
            for x in 0..VIDEO_BUFFER_WIDTH {
                if self.video_buffer[x + y * VIDEO_BUFFER_WIDTH] > 0 {
                    d.draw_rectangle(x as i32 * SCALE, y as i32 * SCALE, SCALE, SCALE, Color::GREEN);
                }
            }
        }
    }

    fn raw(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        Ok(())
    }

    fn cls(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        self.video_buffer.fill(0x00);
        Ok(())
    }

    fn ret(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        if let Some(addr) = self.stack.pop() {
            self.program_counter = addr;
        } else {
            return Err(Keet8Error::StackEmpty);
        }

        Ok(())
    }

    fn sys(&mut self, #[allow(unused)] opcode: OpCode) -> Result<()> {
        Ok(())
    }

    fn jp(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::Addr { address } => self.program_counter = address,
            AddressMode::V0Addr { address } => self.program_counter = self.registers[0x00] as u16 + address,
            _ => return Err(Keet8Error::InvalidAddressMode)
        }

        Ok(())
    }

    fn call(&mut self, opcode: OpCode) -> Result<()> {
        if let AddressMode::Addr { address } = opcode.address_mode {
            self.stack.push(self.program_counter);
            self.program_counter = address;
        } else {
            return Err(Keet8Error::InvalidAddressMode);
        }

        Ok(())
    }

    fn se(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                if self.registers[x as usize] == byte {
                    self.program_counter += 2;
                }
            },
            AddressMode::VxVy { x, y } => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.program_counter += 2;
                }
            },
            _ => return Err(Keet8Error::InvalidAddressMode)
        }
        
        Ok(())
    }

    fn sne(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::VxByte { x, byte } => {
                if self.registers[x as usize] != byte {
                    self.program_counter += 2;
                }
            },
            AddressMode::VxVy { x, y } => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.program_counter += 2;
                }
            },
            _ => return Err(Keet8Error::InvalidAddressMode)
        }

        Ok(())
    }

    fn ld(&mut self, opcode: OpCode) -> Result<()> {
        match opcode.address_mode {
            AddressMode::Addr { address } => todo!(),
            AddressMode::VxByte { x, byte } => {
                self.registers[x as usize] = byte;
            },
            AddressMode::VxVy { x, y } => {
                self.registers[x as usize] = self.registers[y as usize];
            },
            AddressMode::IAddr { address } => {
                self.idx = address;
            },
            AddressMode::V0Addr { address } => todo!(),
            AddressMode::VxVyN { x, y, nibble } => todo!(),
            AddressMode::Vx { x } => todo!(),
            AddressMode::VxDt { x } => {
                self.registers[x as usize] = self.delay_timer;
            },
            AddressMode::VxKey { x } => {
                let mut found = false;
                for i in 0..NUM_KEYS {
                    if self.keypad[i] > 0 {
                        found = true;
                        self.registers[x as usize] = i as u8;
                        break;
                    }
                }

                if !found {
                    self.program_counter -= 2;
                }
            },
            AddressMode::DtVx { x } => {
                self.delay_timer = self.registers[x as usize];
            },
            AddressMode::StVx { x } => {
                self.sound_timer = self.registers[y as usize];
            },
            AddressMode::IVx { x } => todo!(),
            AddressMode::FontVx { x } => {
                let digit = self.registers[x as usize];
                self.idx = 0x0050 + (5 * digit as u16);
            },
            AddressMode::BcdVx { x } => todo!(),
            AddressMode::AddrIVx { address, x } => todo!(),
            AddressMode::VxAddrI { address, x } => todo!(),
            _ => return Err(Keet8Error::InvalidAddressMode)
        }

        todo!()
    }

    fn add(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn or(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn and(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn xor(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn sub(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn shr(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn subn(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn shl(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn rnd(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn drw(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn skp(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }

    fn sknp(&mut self, opcode: OpCode) -> Result<()> {
        todo!()
    }
}