use crate::matrix::transformation::{RotationAmount, TranslationAmount};
use crate::Color;

use glium::glutin::event_loop::ControlFlow;

use std::path::Path;
use std::time::{Duration, Instant};

pub const TEXTURE_PATH: &str = "resources/texture/";
pub const OBJ_PATH: &str = "resources/obj/";

pub struct Setting {
    pub title: &'static str,
    pub icon: &'static Path,
    fps: Option<u8>,
    pub bg_color: Color,
    pub speed: f32,
    pub translation: TranslationAmount,
    pub rotate: RotateAxis,
}

pub enum RotationType {
    Clockwise,
    CounterClockwise,
    None,
}

pub struct RotateAxis {
    pub x: RotationType,
    pub y: RotationType,
    pub z: RotationType,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            title: "scop",
            icon: Path::new("src/icon.png"),
            fps: None,
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            speed: 0.01,
            translation: TranslationAmount {
                x: 0.0,
                y: 0.0,
                z: 6.0,
            },
            rotate: RotateAxis {
                x: RotationType::None,
                y: RotationType::Clockwise,
                z: RotationType::None,
            },
        }
    }
}

impl Setting {
    pub fn fps(&self) -> ControlFlow {
        if let Some(fps) = self.fps {
            ControlFlow::WaitUntil(
                Instant::now() + Duration::from_nanos(1_000_000_000 / fps as u64),
            )
        } else {
            ControlFlow::Poll
        }
    }

    pub fn rotation(&self, rot: &mut RotationAmount) {
        rot.x = self.rotate.x.update(rot.x, self.speed);
        rot.y = self.rotate.y.update(rot.y, self.speed);
        rot.z = self.rotate.z.update(rot.z, self.speed);
    }
}

impl RotationType {
    pub fn update(&self, angle: f32, speed: f32) -> f32 {
        match self {
            RotationType::Clockwise => (angle + speed) % 360.,
            RotationType::CounterClockwise => (angle - speed) % 360.,
            RotationType::None => angle,
        }
    }
}
