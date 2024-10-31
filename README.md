# Keet-8: A Chip-8 emulator

![Rust](https://img.shields.io/badge/Rust-red?style=for-the-badge&logo=Rust&logoColor=white&labelColor=red&color=gray)
![Raylib](https://img.shields.io/badge/Raylib-white?style=for-the-badge&logo=Raylib&logoColor=black&labelColor=white&color=gray)

This is a basic implementation of a [Chip-8](https://en.wikipedia.org/wiki/CHIP-8) interpreter built entirely in Rust. This was done for educational purposes to get started in development with emulators and building them.

## Getting Started

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