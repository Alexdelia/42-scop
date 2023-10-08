use crate::env::Env;
use crate::{Color, ColorPrecision};

use glium::glutin::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, MouseScrollDelta, VirtualKeyCode as VKC, WindowEvent,
    },
    event_loop::ControlFlow,
};

pub enum EventOut {
    None,
    ControlFlow(ControlFlow),
}

impl Env {
    pub fn event(&mut self, event: Event<()>) -> EventOut {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => EventOut::ControlFlow(ControlFlow::Exit),
                WindowEvent::KeyboardInput { input, .. } => self.key(input),
                WindowEvent::CursorMoved { position, .. } => self.cursor(&position),
                WindowEvent::MouseWheel { delta, .. } => self.wheel(delta),
                _ => EventOut::None,
            },
            _ => EventOut::None,
        }
    }

    fn key(&mut self, input: KeyboardInput) -> EventOut {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(key),
            ..
        } = input
        {
            if key == VKC::Escape {
                EventOut::ControlFlow(ControlFlow::Exit)
            } else {
                self.control_key(key);
                EventOut::None
            }
        } else {
            dbg!(input);
            EventOut::None
        }
    }

    fn control_key(&mut self, key: VKC) {
        match key {
            // flow
            VKC::Space => self.setting.speed.pause(),
            VKC::R => self.setting.speed.revert(),
            VKC::Up => {
                self.setting.fps.next();
                self.setting.print_fps();
            }
            VKC::Down => {
                self.setting.fps.prev();
                self.setting.print_fps();
            }
            // object
            VKC::Left => {
                self.gpu.object.prev();
            }
            VKC::Right => {
                self.gpu.object.next();
            }
            VKC::E => self.setting.enlighten = !self.setting.enlighten,
            // speed
            VKC::Plus | VKC::Equals => self.setting.speed.inc(),
            VKC::Minus => self.setting.speed.dec(),
            // translation
            VKC::A => self.setting.translation.x -= 0.1,
            VKC::D => self.setting.translation.x += 0.1,
            VKC::W => self.setting.translation.y += 0.1,
            VKC::S => self.setting.translation.y -= 0.1,
            // rotation
            VKC::X => {
                self.setting.rotate.x.next();
            }
            VKC::Y => {
                self.setting.rotate.y.next();
            }
            VKC::Z => {
                self.setting.rotate.z.next();
            }
            // color
            VKC::C => {
                self.gpu.object.get_mut().0.next();
            }
            // texture
            VKC::T => self.setting.textured = !self.setting.textured,
            VKC::Key5 | VKC::Numpad5 => {
                self.gpu.texture.prev();
            }
            VKC::Key6 | VKC::Numpad6 => {
                self.gpu.texture.next();
            }
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("no bind for {key:?}");
            }
        };
    }

    fn cursor(&mut self, position: &PhysicalPosition<f64>) -> EventOut {
        let (w, h) = self.display.get_framebuffer_dimensions();
        let angle = (position.x - w as f64 / 2.0).atan2(position.y - h as f64 / 2.0);
        let hue = angle / std::f64::consts::PI / 2.0 + 0.5;
        let distance =
            ((position.x - w as f64 / 2.0).powi(2) + (position.y - h as f64 / 2.0).powi(2)).sqrt()
                / (w as f64 / 2.0);
        self.setting.bg_color = Color::hsva(
            hue as ColorPrecision,
            1.0 - distance as ColorPrecision,
            (0.25 + (1.0 - distance) / 2.0) as ColorPrecision,
            0.5,
        );
        EventOut::None
    }

    fn wheel(&mut self, delta: MouseScrollDelta) -> EventOut {
        match delta {
            MouseScrollDelta::LineDelta(_x, y) => {
                if y > 0.0 {
                    self.setting.translation.z /= 1.2;
                } else {
                    self.setting.translation.z *= 1.2;
                }
            }
            MouseScrollDelta::PixelDelta(position) => {
                if position.y > 0.0 {
                    self.setting.translation.z /= 1.2;
                } else {
                    self.setting.translation.z *= 1.2;
                }
            }
        };
        EventOut::None
    }
}
