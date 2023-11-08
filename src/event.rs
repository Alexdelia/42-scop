use crate::{setting::RotationType, Color, ColorPrecision, Env};

use glium::glutin::event::ModifiersState as MS;
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
                WindowEvent::KeyboardInput { input, .. } => self.key(input, self.setting.modifier),
                WindowEvent::CursorMoved { position, .. } => self.cursor(&position),
                WindowEvent::MouseWheel { delta, .. } => self.wheel(delta, self.setting.modifier),
                WindowEvent::ModifiersChanged(modifier) => {
                    self.setting.modifier = modifier;
                    EventOut::None
                }
                _ => EventOut::None,
            },
            _ => EventOut::None,
        }
    }

    fn key(&mut self, input: KeyboardInput, modifier: MS) -> EventOut {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(key),
            ..
        } = input
        {
            if modifier == MS::empty() && (key == VKC::Escape || key == VKC::Q) {
                EventOut::ControlFlow(ControlFlow::Exit)
            } else {
                self.control_key(modifier, key);
                EventOut::None
            }
        } else {
            EventOut::None
        }
    }

    fn control_key(&mut self, modifier: MS, key: VKC) {
        match key {
            // flow
            VKC::Space if modifier == MS::empty() => self.setting.speed.pause(),
            VKC::R if modifier == MS::empty() => self.setting.speed.revert(),
            // speed
            // also VKC::Equals to not bother about the keyboard layout
            VKC::Plus | VKC::Equals if modifier == MS::empty() => self.setting.speed.inc(),
            VKC::Minus if modifier == MS::empty() => self.setting.speed.dec(),
            VKC::Plus | VKC::Equals if modifier == MS::ALT => {
                self.setting.fps.next();
                self.setting.print_fps();
            }
            VKC::Minus if modifier == MS::ALT => {
                self.setting.fps.prev();
                self.setting.print_fps();
            }
            // object
            VKC::Left if modifier == MS::empty() => {
                self.gpu.object.prev();
            }
            VKC::Right if modifier == MS::empty() => {
                self.gpu.object.next();
            }
            // translation
            VKC::A if modifier == MS::empty() => self.setting.translation.x -= 0.1,
            VKC::D if modifier == MS::empty() => self.setting.translation.x += 0.1,
            VKC::W if modifier == MS::empty() => self.setting.translation.y += 0.1,
            VKC::S if modifier == MS::empty() => self.setting.translation.y -= 0.1,
            // rotation
            VKC::X if modifier == MS::empty() => self.setting.rotate.x = RotationType::Clockwise,
            VKC::X if modifier == MS::CTRL => {
                self.setting.rotate.x = RotationType::CounterClockwise
            }
            VKC::X if modifier == MS::SHIFT => self.setting.rotate.x = RotationType::None,
            VKC::Y if modifier == MS::empty() => self.setting.rotate.y = RotationType::Clockwise,
            VKC::Y if modifier == MS::CTRL => {
                self.setting.rotate.y = RotationType::CounterClockwise
            }
            VKC::Y if modifier == MS::SHIFT => self.setting.rotate.y = RotationType::None,
            VKC::Z if modifier == MS::empty() => self.setting.rotate.z = RotationType::Clockwise,
            VKC::Z if modifier == MS::CTRL => {
                self.setting.rotate.z = RotationType::CounterClockwise
            }
            VKC::Z if modifier == MS::SHIFT => self.setting.rotate.z = RotationType::None,
            // light
            VKC::E | VKC::L if modifier == MS::empty() => {
                self.setting.enlighten = !self.setting.enlighten
            }
            // color
            VKC::C if modifier == MS::empty() => self.color_change(true),
            VKC::C if modifier == MS::CTRL => self.color_change(false),
            // texture
            VKC::T if modifier == MS::empty() => self.setting.textured = !self.setting.textured,
            VKC::Left if modifier == MS::ALT => {
                self.gpu.texture.prev();
            }
            VKC::Right if modifier == MS::ALT => {
                self.gpu.texture.next();
            }
            _ => {
                #[cfg(debug_assertions)]
                eprintln!("no bind for {modifier:?} + {key:?}");
            }
        }
    }

    fn color_change(&mut self, next: bool) {
        let c_len = self.gpu.object.get().0.vec.len();
        let cf_len = self.setting.color_face.0.vec.len();
        let len = c_len + cf_len;
        let i = (self.gpu.object.get().0.i
            + self.setting.color_face.0.i
            + if next { 1 } else { len - 1 })
            % len;

        if i < c_len {
            self.gpu.object.get_mut().0.i = i;
            self.setting.color_face.0.i = 0;
            self.setting.color_face.1 = false;
        } else {
            self.gpu.object.get_mut().0.i = self.gpu.object.get().0.vec.len();
            self.setting.color_face.0.i = i - c_len;
            self.setting.color_face.1 = true;
        }
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

    fn wheel(&mut self, delta: MouseScrollDelta, modifier: MS) -> EventOut {
        if modifier != MS::empty() {
            return EventOut::None;
        }

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
