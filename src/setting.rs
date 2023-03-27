use crate::color::Color;

use glium::glutin::event_loop::ControlFlow;

use std::time::{Duration, Instant};

pub struct Setting {
    pub window_title: &'static str,
    pub window_icon: &'static str,
    fps: Option<u8>,
    pub bg_color: Color,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            window_title: "scop",
            window_icon: "src/icon.png",
            fps: None,
            bg_color: Color::new(1.0, 0.717647059, 0.77254902, 0.5),
        }
    }
}

impl Setting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fps(&self) -> ControlFlow {
        if let Some(fps) = self.fps {
            ControlFlow::WaitUntil(
                Instant::now() + Duration::from_nanos(1_000_000_000 / fps as u64),
            )
        } else {
            ControlFlow::Poll
        }
    }
}
