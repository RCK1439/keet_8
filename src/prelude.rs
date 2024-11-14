pub use crate::error::Keet8Error;

// --- custom result type definition ------------------------------------------

/// This is the `Result` type to be used in our library for the emulator
pub type Result<T> = core::result::Result<T, Keet8Error>;
