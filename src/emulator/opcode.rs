use std::fmt::Display;

// --- macros -----------------------------------------------------------------

macro_rules! instr {
    ($raw:expr) => {
        (($raw) & 0xF000)
    };
}

macro_rules! x {
    ($raw:expr) => {
        ((($raw) & 0x0F00) >> 8) as u8
    };
}

macro_rules! y {
    ($raw:expr) => {
        ((($raw) & 0x00F0) >> 4) as u8
    };
}

macro_rules! n {
    ($raw:expr) => {
        (($raw) & 0x000F) as u8
    };
}

macro_rules! kk {
    ($raw:expr) => {
        (($raw) & 0x00FF) as u8
    };
}

macro_rules! nnn {
    ($raw:expr) => {
        (($raw) & 0x0FFF) as u16
    };
}

// --- instruction definition -------------------------------------------------

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Instruction {
    RAW,
    CLS,
    RET,
    #[allow(unused)] SYS,
    JP,
    CALL,
    SE,
    SNE,
    LD,
    ADD,
    OR,
    AND,
    XOR,
    SUB,
    SHR,
    SUBN,
    SHL,
    RND,
    DRW,
    SKP,
    SKNP
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::RAW => write!(f, "raw"),
            Instruction::CLS => write!(f, "cls"),
            Instruction::RET => write!(f, "ret"),
            Instruction::SYS => write!(f, "sys"),
            Instruction::JP => write!(f, "jp"),
            Instruction::CALL => write!(f, "call"),
            Instruction::SE => write!(f, "se"),
            Instruction::SNE => write!(f, "sne"),
            Instruction::LD => write!(f, "ld"),
            Instruction::ADD => write!(f, "add"),
            Instruction::OR => write!(f, "or"),
            Instruction::AND => write!(f, "and"),
            Instruction::XOR => write!(f, "xor"),
            Instruction::SUB => write!(f, "sub"),
            Instruction::SHR => write!(f, "shr"),
            Instruction::SUBN => write!(f, "subn"),
            Instruction::SHL => write!(f, "shl"),
            Instruction::RND => write!(f, "rnd"),
            Instruction::DRW => write!(f, "drw"),
            Instruction::SKP => write!(f, "skp"),
            Instruction::SKNP => write!(f, "sknp"),
        }
    }
}

// --- address mode definition ------------------------------------------------

#[derive(Clone, Copy)]
pub enum AddressMode {
    None,
    OpCode{ opcode: u16 },
    Addr{ address: u16 },
    VxByte{ x: u8, byte: u8 },
    VxVy{ x: u8, y: u8 },
    IAddr{ address: u16 },
    V0Addr{ address: u16 },
    VxVyN{ x: u8, y: u8, nibble: u8 },
    Vx{ x: u8 },
    VxDt{ x: u8 },
    VxKey{ x: u8 },
    DtVx{ x: u8 },
    StVx{ x: u8 },
    IVx{ x: u8 },
    FontVx{ x: u8 },
    BcdVx{ x: u8 },
    AddrIVx{ x: u8 },
    VxAddrI{ x: u8 }
}

impl Display for AddressMode {
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
    pub instr: Instruction,
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
            address_mode: AddressMode::OpCode { opcode }
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
                    address_mode: AddressMode::None
                },
                0x00EE => Self {
                    instr: Instruction::RET,
                    address_mode: AddressMode::None,
                },
                _ => Self::raw(raw)
            },
            0x1000 => Self {
                instr: Instruction::JP,
                address_mode: AddressMode::Addr{ address: nnn!(raw) }
            },
            0x2000 => Self {
                instr: Instruction::CALL,
                address_mode: AddressMode::Addr{ address: nnn!(raw) }
            },
            0x3000 => Self {
                instr: Instruction::SE,
                address_mode: AddressMode::VxByte { x: x!(raw), byte: kk!(raw) }
            },
            0x4000 => Self {
                instr: Instruction::SNE,
                address_mode: AddressMode::VxByte{ x: x!(raw), byte: kk!(raw) }
            },
            0x5000 => Self {
                instr: Instruction::SE,
                address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
            },
            0x6000 => Self {
                instr: Instruction::LD,
                address_mode: AddressMode::VxByte{ x: x!(raw), byte: kk!(raw) }
            },
            0x7000 => Self {
                instr: Instruction::ADD,
                address_mode: AddressMode::VxByte{ x: x!(raw), byte: kk!(raw) }
            },
            0x8000 => match raw & 0x000F {
                0x0000 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0001 => Self {
                    instr: Instruction::OR,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0002 => Self {
                    instr: Instruction::AND,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0003 => Self {
                    instr: Instruction::XOR,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0004 => Self {
                    instr: Instruction::ADD,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0005 => Self {
                    instr: Instruction::SUB,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0006 => Self {
                    instr: Instruction::SHR,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x0007 => Self {
                    instr: Instruction::SUBN,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                0x000E => Self {
                    instr: Instruction::SHL,
                    address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
                },
                _ => Self::raw(raw)
            },
            0x9000 => Self {
                instr: Instruction::SNE,
                address_mode: AddressMode::VxVy{ x: x!(raw), y: y!(raw) }
            },
            0xA000 => Self {
                instr: Instruction::LD,
                address_mode: AddressMode::IAddr{ address: nnn!(raw) }
            },
            0xB000 => Self {
                instr: Instruction::JP,
                address_mode: AddressMode::V0Addr { address: nnn!(raw) }
            },
            0xC000 => Self {
                instr: Instruction::RND,
                address_mode: AddressMode::VxByte { x: x!(raw), byte: kk!(raw) }
            },
            0xD000 => Self {
                instr: Instruction::DRW,
                address_mode: AddressMode::VxVyN { x: x!(raw), y: y!(raw), nibble: n!(raw) }
            },
            0xE000 => match raw & 0x00FF {
                0x0091 => Self {
                    instr: Instruction::SKP,
                    address_mode: AddressMode::Vx { x: x!(raw) }
                },
                0x00A1 => Self {
                    instr: Instruction::SKNP,
                    address_mode: AddressMode::Vx { x: x!(raw) }
                },
                _ => Self::raw(raw)
            },
            0xF000 => match raw & 0x00FF {
                0x0007 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxDt{ x: x!(raw) }
                },
                0x000A => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxKey { x: x!(raw) }
                },
                0x0015 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::DtVx { x: x!(raw) }
                },
                0x0018 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::StVx { x: x!(raw) }
                },
                0x001E => Self {
                    instr: Instruction::ADD,
                    address_mode: AddressMode::IVx { x: x!(raw) }
                },
                0x0029 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::FontVx { x: x!(raw) }
                },
                0x0033 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::BcdVx { x: x!(raw) }
                },
                0x0055 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::AddrIVx { x: x!(raw) }
                },
                0x0065 => Self {
                    instr: Instruction::LD,
                    address_mode: AddressMode::VxAddrI { x: x!(raw) }
                },
                _ => Self::raw(raw)
            },
            _ => Self::raw(raw)
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.instr, self.address_mode)
    }
}