extern crate glium;

use glium::glutin;

pub struct GraphicsManager {
    events_loop: glutin::EventsLoop,
    display: Option<glium::Display>,
}

impl GraphicsManager {
    pub fn new() -> GraphicsManager {
        let mut graphics_manager = GraphicsManager {
            events_loop: glutin::EventsLoop::new(),
            display: None,
        };
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new();
        graphics_manager.display = Some(glium::Display::new(window,
                                                   context,
                                                   &graphics_manager.events_loop).unwrap());
        return graphics_manager;
    }

    pub fn draw_graphics(&self,
        _screen_buffer: [u8; super::chip::SCREEN_WIDTH * super::chip::SCREEN_HEIGHT])
    {
        // TODO
    }
}
