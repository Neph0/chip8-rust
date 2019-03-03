extern crate glium;

use glium::glutin;

pub struct GraphicsManager {
    events_loop: glutin::EventsLoop,
    display: Option<glium::Display>,
}

impl GraphicsManager {
    pub fn new() -> GraphicsManager {
        let mut graphicsManager = GraphicsManager {
            events_loop: glutin::EventsLoop::new(),
            display: None,
        };
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        graphicsManager.display = Some(glium::Display::new(window,
                                                   context,
                                                   &graphicsManager.events_loop).unwrap());
        return graphicsManager;
    }

    //graphicsManager.drawGraphics(chip.graphics);
    pub fn drawGraphics(&self,
        screenBuffer: [u8; super::chip::SCREEN_WIDTH * super::chip::SCREEN_HEIGHT])
    {
        // TODO
    }
}
