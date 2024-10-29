pub use crate::error::Keet8Error;

// --- custom result type definition ------------------------------------------

pub type Result<T> = core::result::Result<T, Keet8Error>;