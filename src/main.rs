//! Keet-8 is a basic Chip-8 emulator built for educational purposes in the
//! world of building emulators. This is my first project in emulation.
//! 
//! To run the emulator, one must provide command-line arguments as follows:
//! 
//! # Examples
//! 
//! `cargo run path/to/rom`

use std::process::ExitCode;

// --- main routine -----------------------------------------------------------

/// The main entry point of the program
fn main() -> ExitCode {
    let args = std::env::args().collect();

    if let Err(e) = keet_8::run(args) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
