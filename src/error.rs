use std::fmt::Display;

pub enum Keet8Error {
    NoROMFile,
}

impl Display for Keet8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keet8Error::NoROMFile => write!(f, "No ROM file specified"),
        }
    }
}
