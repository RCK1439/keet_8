mod error;
mod prelude;
mod emulator;

use raylib::prelude::*;

use error::Keet8Error; 
use prelude::*;

pub fn run(args: Vec<String>) -> Result<()> {
    if args.len() < 2 {
        return Err(Keet8Error::NoROMFile);
    }

    let mut app = Application::new();
    app.run()
}

struct Application {
    rl: RaylibHandle,
    thread: RaylibThread,

    is_running: bool,
    debug: bool,
}

impl Application {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(1024, 512)
            .title("keet-8")
            .vsync()
            .msaa_4x()
            .build();

        rl.set_target_fps(60);

        Self {
            rl,
            thread,

            is_running: true,
            debug: false
        }
    }

    pub fn run(&mut self) -> Result<()> {
        while self.is_running {
            self.on_update()?;
            self.on_render()?;
        }
        
        Ok(())
    }

    fn on_update(&mut self) -> Result<()> {
        if self.rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.is_running = false;
        }

        if self.rl.is_key_pressed(KeyboardKey::KEY_F3) {
            self.debug = !self.debug;
        }
        
        Ok(())
    }

    fn on_render(&mut self) -> Result<()> {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);

        if self.debug {
            d.draw_fps(5, 5);
        }
        
        Ok(())
    }
}