# Keet-8: A Chip-8 emulator

![Rust](https://img.shields.io/badge/Rust-red?style=for-the-badge&logo=Rust&logoColor=white&labelColor=red&color=gray)
![Raylib](https://img.shields.io/badge/raylib-white?style=for-the-badge&logo=raylib&logoColor=black&labelColor=white&color=gray&link=https%3A%2F%2Fwww.raylib.com%2F)

This is a basic implementation of a [Chip-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter built entirely in Rust. This was done for educational purposes to get started in development with emulators and building them.

## Getting Started

### Project Setup

```
keet_8/
│   .gitignore
│   Cargo.lock
│   Cargo.toml
│   README.md
│
├───src/
│   │   error.rs
│   │   lib.rs
│   │   main.rs
│   │   prelude.rs
│   │
│   └───emulator/
│           memory.rs
│           mod.rs
│           opcode.rs
│           stack.rs
│
└───tests/
        1-chip8-logo.ch8
        2-ibm-logo.ch8
        3-corax+.ch8
        4-flags.ch8
        5-quirks.ch8
        6-keypad.ch8
        7-beep.ch8
        8-scrolling.ch8
        chip8-test-rom.ch8
        test_opcode.ch8

```

### Building

 - Clone the repository to your local machine:
 ```bash
 git clone https://github.com/RCK1439/keet_8.git
 ```

 - Change to the root directory of the project:
 ```bash
 cd keet_8/
 ```

 - Run the build command with cargo:
 ```bash
 cargo build --release
 ```

### Running

 - Running the emulator is as simple as follows:
 ```bash
 cargo run --release <rom_path>
 ```

 Where `<rom_path>` is the filepath to a Chip-8 ROM file.

## Dependencies

 - [rand](https://crates.io/crates/rand)
 - [raylib](https://www.raylib.com/)