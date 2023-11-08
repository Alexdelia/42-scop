mod rotation;
pub use rotation::{RotateAxis, RotationType};
mod speed;
use speed::Speed;

use crate::matrix::transformation::{RotationAmount, TranslationAmount};
use crate::{Color, ColorPrecision};

use ivec::IVec;

use ansi::{
    abbrev::{B, BLU, D, F, I, N_C},
    CLEAR_LINE,
};

use glium::glutin::{event::ModifiersState, event_loop::ControlFlow};

use std::path::Path;
use std::time::{Duration, Instant};

pub const TEXTURE_PATH: &str = "resources/texture/";
pub const OBJ_PATH: &str = "resources/obj/";

pub struct Setting {
    pub title: &'static str,
    pub icon: &'static Path,
    pub modifier: ModifiersState,
    pub fps: IVec<Option<u8>>,
    pub bg_color: Color,
    pub speed: Speed,
    pub translation: TranslationAmount,
    pub rotate: RotateAxis,
    pub textured: bool,
    pub color_face: (IVec<ColorFace>, bool),
    pub enlighten: bool,
}

pub struct ColorFace {
    pub colors: [[ColorPrecision; 4]; 5],
    pub len: u32,
}

impl Default for Setting {
    fn default() -> Self {
        Self {
            title: "scop",
            icon: Path::new("src/icon.png"),
            modifier: ModifiersState::default(),
            fps: vec![
                None,
                Some(5),
                Some(10),
                Some(15),
                Some(30),
                Some(60),
                Some(120),
                Some(240),
            ]
            .into(),
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            speed: Speed::default(),
            translation: TranslationAmount {
                x: 0.0,
                y: 0.0,
                z: 2.5,
            },
            rotate: RotateAxis {
                x: RotationType::None,
                y: RotationType::Clockwise,
                z: RotationType::None,
            },
            textured: false,
            color_face: (
                vec![
                    ColorFace {
                        colors: [
                            // 5 shade of gray
                            [0.1, 0.1, 0.1, 1.0],
                            [0.2, 0.2, 0.2, 1.0],
                            [0.3, 0.3, 0.3, 1.0],
                            [0.4, 0.4, 0.4, 1.0],
                            [0.5, 0.5, 0.5, 1.0],
                        ],
                        len: 5,
                    },
                    ColorFace {
                        colors: [
                            // 5 shade of blue-purple
                            [0.1, 0.1, 0.2, 1.0],
                            [0.2, 0.2, 0.4, 1.0],
                            [0.3, 0.3, 0.6, 1.0],
                            [0.4, 0.4, 0.8, 1.0],
                            [0.5, 0.5, 1.0, 1.0],
                        ],
                        len: 5,
                    },
                    ColorFace {
                        colors: [
                            [0.3, 0.3, 0.3, 1.0],
                            [0.7, 0.7, 0.7, 1.0],
                            [0.0, 0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0, 0.0],
                            [0.0, 0.0, 0.0, 0.0],
                        ],
                        len: 2,
                    },
                ]
                .into(),
                false,
            ),
            enlighten: true,
        }
    }
}

impl Setting {
    pub fn control_flow(&self) -> ControlFlow {
        if let Some(fps) = self.fps.get() {
            ControlFlow::WaitUntil(
                Instant::now() + Duration::from_nanos(1_000_000_000 / *fps as u64),
            )
        } else {
            ControlFlow::Poll
        }
    }

    pub fn print_fps(&self) {
        println!(
            "\x1b[A{CLEAR_LINE}\r\t{B}{I}{BLU}{fps} {N_C}{F}fps{D}",
            fps = self
                .fps
                .get()
                .map(|x| x.to_string())
                .unwrap_or("unlimited".to_string())
        );
    }

    pub fn rotation(&self, rot: &mut RotationAmount) {
        rot.x = self.rotate.x.update(rot.x, self.speed.get());
        rot.y = self.rotate.y.update(rot.y, self.speed.get());
        rot.z = self.rotate.z.update(rot.z, self.speed.get());
    }
}
