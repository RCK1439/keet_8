use std::fmt::Display;

// --- macros -----------------------------------------------------------------

macro_rules! instr {
    ($raw:expr) => {
        (($raw) & 0xF000)
    };
}

macro_rules! x {
    ($raw:expr) => {
        ((($raw) & 0x0F00) >> 8) as usize
    };
}

macro_rules! y {
    ($raw:expr) => {
        ((($raw) & 0x00F0) >> 4) as usize
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
        const INSTRUCTION_STRINGS: [&'static str; 21] = [
            "raw", "cls", "ret", "sys", "jp", "call", "se",
            "sne", "ld", "add", "or", "and", "xor", "sub",
            "shr", "subn", "shl", "rnd", "drw", "skp", "sknp"
        ];

        write!(f, "{}", INSTRUCTION_STRINGS[*self as usize])
    }
}

// --- address mode definition ------------------------------------------------

#[derive(Clone, Copy)]
pub enum AddressMode {
    None,
    OpCode{ opcode: u16 },
    Addr{ address: u16 },
    VxByte{ x: usize, byte: u8 },
    VxVy{ x: usize, y: usize },
    IAddr{ address: u16 },
    V0Addr{ address: u16 },
    VxVyN{ x: usize, y: usize, nibble: u8 },
    Vx{ x: usize },
    VxDt{ x: usize },
    VxKey{ x: usize },
    DtVx{ x: usize },
    StVx{ x: usize },
    IVx{ x: usize },
    FontVx{ x: usize },
    BcdVx{ x: usize },
    AddrIVx{ x: usize },
    VxAddrI{ x: usize }
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
