use crate::setting::Setting;

use glium::{backend::glutin::DisplayCreationError, glutin};

pub struct Env {
    pub display: glium::Display,
    pub setting: Setting,
}

impl Env {
    pub fn new(
        event_loop: &glutin::event_loop::EventLoop<()>,
    ) -> Result<Self, DisplayCreationError> {
        let display = glium::Display::new(
            glutin::window::WindowBuilder::new(),
            glutin::ContextBuilder::new(),
            &event_loop,
        )?;

        Ok(Self {
            display: glium::Display::new(
                glutin::window::WindowBuilder::new(),
                glutin::ContextBuilder::new(),
                &event_loop,
            )?,
            setting: Setting::default(),
        })
    }
}
