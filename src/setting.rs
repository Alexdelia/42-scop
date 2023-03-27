use glium::glutin::event_loop::ControlFlow;

use std::time::{Duration, Instant};

// pub const WINDOW_INIT_COLOR: wgpu::Color = wgpu::Color {
//     r: 1.0,
//     g: 0.717647059,
//     b: 0.77254902,
//     a: 0.5,
// };

pub struct Setting {
    pub window_title: &'static str,
    pub window_icon: &'static str,
    fps: Option<u8>,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            window_title: "scop",
            window_icon: "src/icon.png",
            fps: None,
        }
    }
}

impl Setting {
    pub fn new() -> Self {
        Self::default()
    }

    fn fps(&self) -> ControlFlow {
        if let Some(fps) = self.fps {
            ControlFlow::WaitUntil(
                Instant::now() + Duration::from_nanos(1_000_000_000 / fps as u64),
            )
        } else {
            ControlFlow::Poll
        }
    }
}
