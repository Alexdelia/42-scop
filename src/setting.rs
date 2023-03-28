use crate::Color;

use glium::glutin::event_loop::ControlFlow;

use std::path::Path;
use std::time::{Duration, Instant};

pub struct Setting {
    pub title: &'static str,
    pub icon: &'static Path,
    fps: Option<u8>,
    pub bg_color: Color,
    pub texture: Option<&'static Path>,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            title: "scop",
            icon: Path::new("src/icon.png"),
            fps: None,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            texture: Some(Path::new("resources/texture/mlp0.png")),
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
