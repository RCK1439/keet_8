//! This module, `keet_8`, contains the actual implementation of our
//! application.
//! 
//! This is done purposely seperately from the main binary crate to have a form
//! of abstraction from the main routine.
//! 
//! This module only exposes one function, that being the `run` function, which
//! is to be called from `main`.

mod emulator;
mod error;
mod prelude;

use emulator::Emulator;
use prelude::*;

use raylib::prelude::*;

// --- keet-8 interface -------------------------------------------------------

/// Runs the application
///
/// # Params
///
/// - `args` - The command-line arguments
/// 
/// # Examples
/// 
/// ```rust
/// let args = std::env::args()
///     .collect();
/// 
/// if let Err(e) = keet_8::run(args) {
///     eprintln!("{e}");
/// }
/// ```
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

// --- constants --------------------------------------------------------------

/// Represents the title of the emulator
const TITLE: &'static str = "Keet-8";
/// Represents the current version of the emulator
const VERSION: &'static str = "v1.0.0";

/// Represents the width of the window
const WINDOW_WIDTH: i32 = 1024;
/// Represents the height of the window
const WINDOW_HEIGHT: i32 = 512;

/// The delay in seconds between CPU cycles for the emulator (60FPS or 16.67ms)
const EMU_STEP_DELAY: f32 = 1.0 / 60.0;

// --- application definition -------------------------------------------------

struct Application {
    /// The handle to the raylib context
    rl: RaylibHandle,
    /// The thread on which raylib is running on
    thread: RaylibThread,
    /// Flag indicating whether the application is still running
    is_running: bool,
    /// Flag indicating whether debug information is to be drawn on the window
    debug: bool,
    /// The actual Chip-8 emulator
    emulator: Emulator,
    /// The current time in seconds for the CPU ticks
    curr_time: f32,
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
        let (mut rl, thread) = if cfg!(debug_assertions) {
            let window_title = format!("{TITLE} - {VERSION} (debug)");
            raylib::init()
                .size(WINDOW_WIDTH, WINDOW_HEIGHT)
                .title(&window_title)
                .vsync()
                .msaa_4x()
                .resizable()
                .build()

        // We don't want logging for release builds
        } else {
            let window_title = format!("{TITLE} - {VERSION}");
            raylib::init()
                .size(WINDOW_WIDTH, WINDOW_HEIGHT)
                .title(&window_title)
                .vsync()
                .msaa_4x()
                .resizable()
                .log_level(TraceLogLevel::LOG_NONE)
                .build()
        };

        rl.set_window_min_size(WINDOW_WIDTH, WINDOW_HEIGHT);

        Ok(Self {
            rl,
            thread,
            is_running: true,
            debug: false,
            emulator: Emulator::new(rom_file)?,
            curr_time: 0.0
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
        // Step the emulator if timer has met the delay time 
        if self.curr_time >= EMU_STEP_DELAY {
            self.process_input();
            self.emulator.step()?;

            self.curr_time -= EMU_STEP_DELAY;

        // Otherwise accumelate the timer
        } else {
            self.curr_time += self.rl.get_frame_time();
        }

        // Close the application if the escape key has been pressed
        if self.rl.window_should_close() {
            self.is_running = false;
        }

        // Show debugging information when F3 has been pressed (like Minecraft)
        if self.rl.is_key_pressed(KeyboardKey::KEY_F3) {
            self.debug = !self.debug;
        }

        // Make the window fullsreen when F11 is pressed
        if self.rl.is_key_pressed(KeyboardKey::KEY_F11) {
            if self.rl.is_window_fullscreen() {
                self.rl.toggle_fullscreen();
            } else {
                let monitor = raylib::window::get_current_monitor();
                let width = raylib::window::get_monitor_width(monitor);
                let height = raylib::window::get_monitor_height(monitor);

                self.rl.set_window_size(width, height);
                self.rl.toggle_fullscreen();
            }
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
            KeyboardKey::KEY_F,
        ];

        (0..NUM_KEYS).for_each(|k| {
            self.emulator
                .set_key(k, self.rl.is_key_down(KEYBOARD_KEY[k]) as u8)
        });
    }
}
