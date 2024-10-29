mod error;
mod prelude;
mod emulator;

use raylib::prelude::*;

use emulator::Emulator;
use prelude::*;

// --- keet-8 interface -------------------------------------------------------

/// Runs the application
/// 
/// # Params
/// 
/// - `args` - The command-line arguments
/// 
/// # Errors
/// 
/// - If no ROM file was provided
/// - If there was an error when loading the ROM
/// - If there was an error during runtime
pub fn run(args: Vec<String>) -> Result<()> {
    if args.len() < 2 {
        return Err(Keet8Error::NoROMFile);
    }

    let mut app = Application::new(&args[1])?;
    app.run()
}

// --- application definition -------------------------------------------------

struct Application {
    rl: RaylibHandle,
    thread: RaylibThread,

    is_running: bool,
    debug: bool,

    emulator: Emulator
}

impl Application {
    /// Creates an instance of the application and initializes raylib
    /// 
    /// # Params
    /// 
    /// - `rom_file` - The filepath to the ROM file
    /// 
    /// # Errors
    /// 
    /// If an error occured when loading the ROM file
    pub fn new(rom_file: &str) -> Result<Self> {
        let (mut rl, thread) = raylib::init()
            .size(1024, 512)
            .title("keet-8")
            .vsync()
            .msaa_4x()
            .build();

        rl.set_target_fps(60);

        Ok(Self {
            rl,
            thread,

            is_running: true,
            debug: false,

            emulator: Emulator::new(rom_file)?
        })
    }

    /// Runs the application
    /// 
    /// # Errors
    /// 
    /// If an error occured during runtime of the emulator
    pub fn run(&mut self) -> Result<()> {
        while self.is_running {
            self.on_update()?;
            self.on_render();
        }
        
        Ok(())
    }

    /// Called once per frame to update the logic of the application
    /// 
    /// # Errors
    /// 
    /// If an error has occured during runtime of the emulator
    fn on_update(&mut self) -> Result<()> {
        self.process_input();
        self.emulator.step()?;

        if self.rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.is_running = false;
        }

        if self.rl.is_key_pressed(KeyboardKey::KEY_F3) {
            self.debug = !self.debug;
        }
        
        Ok(())
    }

    /// Called once per frame to draw everything to the window
    fn on_render(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        self.emulator.draw_buffer(&mut d);
        if self.debug {
            d.draw_fps(5, 5);
        }
    }

    /// Processes the keyboard input
    fn process_input(&mut self) {
        const NUM_KEYS: usize = 16;
        const KEYBOARD_KEY: [KeyboardKey; NUM_KEYS] = [
            KeyboardKey::KEY_ZERO,
            KeyboardKey::KEY_ONE,
            KeyboardKey::KEY_TWO,
            KeyboardKey::KEY_THREE,
            KeyboardKey::KEY_FOUR,
            KeyboardKey::KEY_FIVE,
            KeyboardKey::KEY_SIX,
            KeyboardKey::KEY_SEVEN,
            KeyboardKey::KEY_EIGHT,
            KeyboardKey::KEY_NINE,
            KeyboardKey::KEY_A,
            KeyboardKey::KEY_B,
            KeyboardKey::KEY_C,
            KeyboardKey::KEY_D,
            KeyboardKey::KEY_E,
            KeyboardKey::KEY_F
        ];

        for k in 0..NUM_KEYS {
            if self.rl.is_key_down(KEYBOARD_KEY[k]) {
                self.emulator.set_key(k as u8, 1);
            } else {
                self.emulator.set_key(k as u8, 0);
            }
        }
    }
}