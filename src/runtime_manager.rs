extern crate sfml;

use sfml::graphics;
use sfml::window;

use sfml::graphics::{Color, RenderTarget};
use sfml::window::{Event, Key};

const PIXEL_DISPLAY_SIZE: u32 = 1;
const _EVENT_TO_KEYPAD_INDEX: [Key; 16] = [
    Key::Num1, Key::Num2, Key::Num3, Key::Num4,
    Key::Q, Key::W, Key::E, Key::R,
    Key::A, Key::S, Key::D, Key::F,
    Key::Z, Key::X, Key::C, Key::V
];

pub struct RuntimeManager {
    pub window: graphics::RenderWindow,
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
                        window::Style::DEFAULT,
                        &context_settings,
                        )
        }
    }

    pub fn handle_events(&mut self, chip: &mut super::chip::Chip,)
    {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, ..  } => { chip.exit_flag = 1; },
                // What the fuck is this SHIT?
                Event::KeyPressed  { code: Key::Num1, .. } => { chip.set_key(0,   true); }
                Event::KeyPressed  { code: Key::Num2, .. } => { chip.set_key(1,   true); }
                Event::KeyPressed  { code: Key::Num3, .. } => { chip.set_key(2,   true); }
                Event::KeyPressed  { code: Key::Num4, .. } => { chip.set_key(3,   true); }
                Event::KeyPressed  { code: Key::Q,    .. } => { chip.set_key(4,   true); }
                Event::KeyPressed  { code: Key::W,    .. } => { chip.set_key(5,   true); }
                Event::KeyPressed  { code: Key::E,    .. } => { chip.set_key(6,   true); }
                Event::KeyPressed  { code: Key::R,    .. } => { chip.set_key(7,   true); }
                Event::KeyPressed  { code: Key::A,    .. } => { chip.set_key(8,   true); }
                Event::KeyPressed  { code: Key::S,    .. } => { chip.set_key(9,   true); }
                Event::KeyPressed  { code: Key::D,    .. } => { chip.set_key(10,  true); }
                Event::KeyPressed  { code: Key::F,    .. } => { chip.set_key(11,  true); }
                Event::KeyPressed  { code: Key::Z,    .. } => { chip.set_key(12,  true); }
                Event::KeyPressed  { code: Key::X,    .. } => { chip.set_key(13,  true); }
                Event::KeyPressed  { code: Key::C,    .. } => { chip.set_key(14,  true); }
                Event::KeyPressed  { code: Key::V,    .. } => { chip.set_key(15,  true); }
                Event::KeyReleased { code: Key::Num1, .. } => { chip.set_key(0,  false); }
                Event::KeyReleased { code: Key::Num2, .. } => { chip.set_key(1,  false); }
                Event::KeyReleased { code: Key::Num3, .. } => { chip.set_key(2,  false); }
                Event::KeyReleased { code: Key::Num4, .. } => { chip.set_key(3,  false); }
                Event::KeyReleased { code: Key::Q,    .. } => { chip.set_key(4,  false); }
                Event::KeyReleased { code: Key::W,    .. } => { chip.set_key(5,  false); }
                Event::KeyReleased { code: Key::E,    .. } => { chip.set_key(6,  false); }
                Event::KeyReleased { code: Key::R,    .. } => { chip.set_key(7,  false); }
                Event::KeyReleased { code: Key::A,    .. } => { chip.set_key(8,  false); }
                Event::KeyReleased { code: Key::S,    .. } => { chip.set_key(9,  false); }
                Event::KeyReleased { code: Key::D,    .. } => { chip.set_key(10, false); }
                Event::KeyReleased { code: Key::F,    .. } => { chip.set_key(11, false); }
                Event::KeyReleased { code: Key::Z,    .. } => { chip.set_key(12, false); }
                Event::KeyReleased { code: Key::X,    .. } => { chip.set_key(13, false); }
                Event::KeyReleased { code: Key::C,    .. } => { chip.set_key(14, false); }
                Event::KeyReleased { code: Key::V,    .. } => { chip.set_key(15, false); }
                _ => {
                    //println!("Un-handled event: {:?}", event);
                }
            }
        }
    }

    pub fn draw_graphics(&mut self,
         screen_buffer: &[u8; super::chip::SCREEN_WIDTH * super::chip::SCREEN_HEIGHT])
    {
        const W: u32 = super::chip::SCREEN_WIDTH as u32;
        const H: u32 = super::chip::SCREEN_HEIGHT as u32;
        // SFML sprites, 4 bytes per pixel: (r, g, b, a)
        let mut pixels = vec![0; (W * H * 4) as usize];
        //let mut j = 0;
        //for i in 0..(W * H) as usize {
        //    match screen_buffer[i] {
        //        0 => { // BLACK
        //            pixels[j]     = 0;
        //            pixels[j + 1] = 0;
        //            pixels[j + 2] = 0;
        //        },
        //        _ => { // WHITE
        //            pixels[j]     = 255;
        //            pixels[j + 1] = 255;
        //            pixels[j + 2] = 255;
        //        }
        //    }
        //    pixels[j + 3] = 255;
        //    j += 4;
        //}

        for y in 0..H as usize {
            for x in 0..W as usize {
                let pos = ((y * super::chip::SCREEN_WIDTH) + x) * 4;
                match screen_buffer[y * super::chip::SCREEN_WIDTH + x] {
                    0 => { // BLACK
                        pixels[pos]     = 0;
                        pixels[pos + 1] = 0;
                        pixels[pos + 2] = 0;
                    },
                    _ => { // WHITE
                        pixels[pos]     = 255;
                        pixels[pos + 1] = 255;
                        pixels[pos + 2] = 255;
                    }
                }
                pixels[pos + 3] = 255;
            }
        }

        let mut texture = graphics::Texture::new(W, H).unwrap();
        texture.update_from_pixels(&pixels, W, H, 0, 0);
        let sprite  = graphics::Sprite::with_texture(&texture);
        self.window.draw(&sprite);
        self.window.display();
    }

    pub fn clear_screen(&mut self)
    {
        self.window.clear(&Color::rgb(0, 0, 0));
    }
}
