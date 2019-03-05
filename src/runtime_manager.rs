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
            //antialiasing_level: 0,
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
         screen_buffer: &[u8; super::chip::SCREEN_WIDTH * super::chip::SCREEN_HEIGHT])
    {
        self.window.clear(&Color::rgb(0, 0, 0));

        const w: u32 = super::chip::SCREEN_WIDTH as u32;
        const h: u32 = super::chip::SCREEN_HEIGHT as u32;
        let mut pixels = vec![0; (w * h * 4) as usize];
        for i in 0..(w * h) as usize {
            match screen_buffer[i] {
                0 => {
                    pixels[i]     = 0;
                    pixels[i + 1] = 0;
                    pixels[i + 2] = 0;
                },
                _ => {
                    pixels[i]     = 255;
                    pixels[i + 1] = 255;
                    pixels[i + 2] = 255;
                }
            }
            pixels[i + 3] = 255;
        }

        let mut texture = graphics::Texture::new(w, h).unwrap();
        texture.update_from_pixels(&pixels, w, h, 0, 0);
        let sprite  = graphics::Sprite::with_texture(&texture);
        self.window.draw(&sprite);
        self.window.display();
    }
}
