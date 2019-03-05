extern crate sfml;

use sfml::graphics;
use sfml::window;

use sfml::graphics::{Color, RenderTarget};
use sfml::window::{Event, Key};

const PIXEL_DISPLAY_SIZE: u32 = 16;

pub struct RuntimeManager {
    window: graphics::RenderWindow,
}

impl RuntimeManager {
    pub fn new() -> RuntimeManager {
        let context_settings = window::ContextSettings {
            antialiasing_level: 0,
            ..Default::default()
        };
        RuntimeManager {
            window: graphics::RenderWindow::new(
                        ((super::chip::SCREEN_WIDTH as u32) * PIXEL_DISPLAY_SIZE,
                         (super::chip::SCREEN_HEIGHT as u32) * PIXEL_DISPLAY_SIZE),
                        "CHIP8-EMULATOR",
                        window::Style::CLOSE,
                        &context_settings,
                        )
        }
    }

    pub fn handle_events(&mut self, chip: &mut super::chip::Chip)
    {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => {
                    chip.exit_flag = 1;
                },
                Event::TextEntered {..} => {
                    // Ignore text input
                }
                _ => {
                    println!("Un-handled event: {:?}", event);
                }
            }
        }
    }

    pub fn draw_graphics(&mut self,
         _screen_buffer: [u8; super::chip::SCREEN_WIDTH * super::chip::SCREEN_HEIGHT])
    {
        self.window.clear(&Color::rgb(0, 0, 0));
    }
}
