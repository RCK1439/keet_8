use std::fmt::Display;

// --- macros -----------------------------------------------------------------

/// Retrieves the first nibble of the raw opcode
macro_rules! instr {
    ($raw:expr) => {
        (($raw) & 0xF000)
    };
}

/// Retrieves the value of `VX` from the raw opcode
macro_rules! x {
    ($raw:expr) => {
        ((($raw) & 0x0F00) >> 8) as usize
    };
}

/// Retrieves the value of `VY` from the raw opcode
macro_rules! y {
    ($raw:expr) => {
        ((($raw) & 0x00F0) >> 4) as usize
    };
}

/// Retrieves the nibble (`N`) from the raw opcode as a byte
macro_rules! n {
    ($raw:expr) => {
        (($raw) & 0x000F) as u8
    };
}

/// Retrieves the byte (`KK`) from the raw opcode as a byte
macro_rules! kk {
    ($raw:expr) => {
        (($raw) & 0x00FF) as u8
    };
}

/// Retrieves the address (`NNN`) from the raw opcode as a 16-bit unsigned
/// integer
macro_rules! nnn {
    ($raw:expr) => {
        (($raw) & 0x0FFF) as u16
    };
}

// --- instruction definition -------------------------------------------------

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Instruction {
    /// `raw` instruction (used for when an unknown raw opcode was encountered)
    RAW,
    /// `cls` instruction to clear the screen buffer
    CLS,
    /// `ret` intruction for returning from functions
    RET,
    /// `sys` instruction (unused)
    #[allow(unused)]
    SYS,
    /// `jp` instruction for jumping to different memory addresses
    JP,
    /// `call` instruction for calling functions
    CALL,
    /// `se` instruction for skipping the next instruction if two values are
    /// equal
    SE,
    /// `sne` instruction for skipping the next instruction if two value are
    /// not equal
    SNE,
    /// `ld` instruction for loading a value into a register
    LD,
    /// `add` instruction for performing a mathematical add operation
    ADD,
    /// `or` instruction for performing a bitwise "or" operation
    OR,
    /// `and` instruction for performing a bitwise "and" operation
    AND,
    /// `xor` instruction for performing a bitwise "xor" operation
    XOR,
    /// `sub` instruction for performing a mathematical subtract operation
    SUB,
    /// `shr` instruction for performing a bitwise shift to the right operation
    SHR,
    /// `subn` instruction for performing a negative mathematical subtract
    /// operation
    SUBN,
    /// `shl` instruction for performing a bitwise shift to the left operation
    SHL,
    /// `rnd` instruction for generating a random unsigned 8-bit integer 
    RND,
    /// `drw` instruction for drawing to the screen buffer
    DRW,
    /// `skp` instruction for skipping the next instruction if a specific key
    /// is pressed
    SKP,
    /// `sknp` instruction fro skipping the next instruction if a specific key
    /// is not pressed
    SKNP,
}

impl Display for Instruction {
    /// Writes the instruction to the output stream
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const INSTRUCTION_STRINGS: [&'static str; 21] = [
            "raw", "cls", "ret", "sys", "jp", "call", "se", "sne", "ld", "add", "or", "and", "xor",
            "sub", "shr", "subn", "shl", "rnd", "drw", "skp", "sknp",
        ];

        write!(f, "{}", INSTRUCTION_STRINGS[*self as usize])
    }
}

// --- address mode definition ------------------------------------------------

#[derive(Clone, Copy)]
pub enum AddressMode {
    /// Used for instructions that require no address mode
    None,
    /// Used for the `raw` instruction
    OpCode { opcode: u16 },
    /// Used for instructions requiring a memory address
    Addr { address: u16 },
    /// Used for instructions operating on a register with a byte
    VxByte { x: usize, byte: u8 },
    /// Used for instructions operating on two registers
    VxVy { x: usize, y: usize },
    /// Used for instruction requiring a memory address aswell as the index
    /// register
    IAddr { address: u16 },
    /// Used for instructions using the `V0` register and a memory address
    V0Addr { address: u16 },
    /// Used for the `drw` instruction
    VxVyN { x: usize, y: usize, nibble: u8 },
    /// Used for instructions operating on a single register
    Vx { x: usize },
    /// Used for instructions operating on a single register as the destination
    /// and the delay timer
    VxDt { x: usize },
    /// Used for instructions operating on a single register and making use of
    /// a key
    VxKey { x: usize },
    /// Used for instructions operating on the delay timer as the destination
    /// and a single register
    DtVx { x: usize },
    /// Used for instructions operating on the sound timer as the destination
    /// and a single register
    StVx { x: usize },
    /// Used for instructions operating on the index register as the
    /// destination and a single register
    IVx { x: usize },
    /// Used for instructions operating on the font and a single register
    FontVx { x: usize },
    /// Used for instructions operating on a single register with a weird
    /// operation
    BcdVx { x: usize },
    /// Used for instructions operating on a memory address and the index
    /// register as the destination and a single other register
    AddrIVx { x: usize },
    /// Used for instructions operating on a single register as the destination
    /// and a memory address with the index register
    VxAddrI { x: usize },
}

impl Display for AddressMode {
    /// Writes the address mode as it will appear in assembly to the output
    /// stream
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddressMode::None => write!(f, ""),
            AddressMode::OpCode { opcode } => write!(f, "0x{:04x}", opcode),
            AddressMode::Addr { address } => write!(f, "0x{:04x}", address),
            AddressMode::VxByte { x, byte } => write!(f, "v{x} {byte}"),
            AddressMode::VxVy { x, y } => write!(f, "v{x} v{y}"),
            AddressMode::IAddr { address } => write!(f, "I 0x{:04x}", address),
            AddressMode::V0Addr { address } => write!(f, "v0 0x{:04x}", address),
            AddressMode::VxVyN { x, y, nibble } => write!(f, "v{x} v{y} {nibble}"),
            AddressMode::Vx { x } => write!(f, "v{x}"),
            AddressMode::VxDt { x } => write!(f, "v{x} dt"),
            AddressMode::VxKey { x } => write!(f, "v{x} [key]"),
            AddressMode::DtVx { x } => write!(f, "dt v{x}"),
            AddressMode::StVx { x } => write!(f, "st v{x}"),
            AddressMode::IVx { x } => write!(f, "I v{x}"),
            AddressMode::FontVx { x } => write!(f, "v{x}"),
            AddressMode::BcdVx { x } => write!(f, "v{x}"),
            AddressMode::AddrIVx { x } => write!(f, "v{x}"),
            AddressMode::VxAddrI { x } => write!(f, "v{x}"),
        }
    }
}

// --- opcode definition ------------------------------------------------------

#[derive(Clone, Copy)]
pub struct OpCode {
    /// The specified instruction
    pub instr: Instruction,
    /// The address mode to treat the instruction with
    pub address_mode: AddressMode,
}

impl OpCode {
    /// Creates an opcode from a raw opcode found in the ROM binary
    ///
    /// # Params
    ///
    /// - `opcode` - The raw binary opcode in the ROM file
    fn raw(opcode: u16) -> Self {
        Self {
            instr: Instruction::RAW,
            address_mode: AddressMode::OpCode { opcode },
        }
    }
}

impl From<u16> for OpCode {
    /// Creates an opcode struct from the raw binary opcode found in the ROM
    /// file
    ///
    /// # Params
    ///
    /// - `raw` - The raw binary opcode
    fn from(raw: u16) -> Self {
        match instr!(raw) {
            0x0000 => match raw & 0x00FF {
                0x00E0 => Self {
                    instr: Instruction::CLS,
                    address_mode: AddressMode::None,
                },
                0x00EE => Self {
                    instr: Instruction::RET,
                    address_mode: AddressMode::None,
                },
                _ => Self::raw(raw),
            },
            0x1000 => Self {
                instr: Instruction::JP,
                address_mode: AddressMode::Addr { address: nnn!(raw) },
            },
            0x2000 => Self {
                instr: Instruction::CALL,
                address_mode: AddressMode::Addr { address: nnn!(raw) },
            },
            0x3000 => Self {
                instr: Instruction::SE,
                address_mode: AddressMode::VxByte {
                    x: x!(raw),
                    byte: kk!(raw),
                },
            },
            0x4000 => Self {
                instr: Instruction::SNE,
                address_mode: AddressMode::VxByte {
                    x: x!(raw),
                    byte: kk!(raw),
                },
            },
            0x5000 => Self {
                instr: Instruction::SE,
                address_mode: AddressMode::VxVy {
                    x: x!(raw),
                    y: y!(raw),
                },
            },
            0x6000 => Self {
                instr: Instruction::LD,
                address_mode: AddressMode::VxByte {
                    x: x!(raw),
                    byte: kk!(raw),
                },
            },
            0x7000 => Self {
                instr: Instruction::ADD,
                address_mode: AddressMode::VxByte {
                    x: x!(raw),
                    byte: kk!(raw),
                },
            },
            0x8000 => match raw & 0x000F {
                0x0000 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0001 => Self {
                    instr: Instruction::OR,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0002 => Self {
                    instr: Instruction::AND,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0003 => Self {
                    instr: Instruction::XOR,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0004 => Self {
                    instr: Instruction::ADD,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0005 => Self {
                    instr: Instruction::SUB,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0006 => Self {
                    instr: Instruction::SHR,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x0007 => Self {
                    instr: Instruction::SUBN,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                0x000E => Self {
                    instr: Instruction::SHL,
                    address_mode: AddressMode::VxVy {
                        x: x!(raw),
                        y: y!(raw),
                    },
                },
                _ => Self::raw(raw),
            },
            0x9000 => Self {
                instr: Instruction::SNE,
                address_mode: AddressMode::VxVy {
                    x: x!(raw),
                    y: y!(raw),
                },
            },
            0xA000 => Self {
                instr: Instruction::LD,
                address_mode: AddressMode::IAddr { address: nnn!(raw) },
            },
            0xB000 => Self {
                instr: Instruction::JP,
                address_mode: AddressMode::V0Addr { address: nnn!(raw) },
            },
            0xC000 => Self {
                instr: Instruction::RND,
                address_mode: AddressMode::VxByte {
                    x: x!(raw),
                    byte: kk!(raw),
                },
            },
            0xD000 => Self {
                instr: Instruction::DRW,
                address_mode: AddressMode::VxVyN {
                    x: x!(raw),
                    y: y!(raw),
                    nibble: n!(raw),
                },
            },
            0xE000 => match raw & 0x00FF {
                0x0091 => Self {
                    instr: Instruction::SKP,
                    address_mode: AddressMode::Vx { x: x!(raw) },
                },
                0x00A1 => Self {
                    instr: Instruction::SKNP,
                    address_mode: AddressMode::Vx { x: x!(raw) },
                },
                _ => Self::raw(raw),
            },
            0xF000 => match raw & 0x00FF {
                0x0007 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxDt { x: x!(raw) },
                },
                0x000A => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxKey { x: x!(raw) },
                },
                0x0015 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::DtVx { x: x!(raw) },
                },
                0x0018 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::StVx { x: x!(raw) },
                },
                0x001E => Self {
                    instr: Instruction::ADD,
                    address_mode: AddressMode::IVx { x: x!(raw) },
                },
                0x0029 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::FontVx { x: x!(raw) },
                },
                0x0033 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::BcdVx { x: x!(raw) },
                },
                0x0055 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::AddrIVx { x: x!(raw) },
                },
                0x0065 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxAddrI { x: x!(raw) },
                },
                _ => Self::raw(raw),
            },
            _ => Self::raw(raw),
        }
    }
}

impl Display for OpCode {
    /// Writes the opcode out as it will appear in assembly to the
    /// output stream
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.instr, self.address_mode)
    }
}
