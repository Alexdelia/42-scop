use crate::env::Env;
use crate::{Color, ColorPrecision};

use glium::glutin::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
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
                WindowEvent::CloseRequested => return EventOut::ControlFlow(ControlFlow::Exit),
                WindowEvent::KeyboardInput { input, .. } => self.key(input),
                WindowEvent::CursorMoved { position, .. } => self.cursor(&position),
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
            self.key_simple(key)
        } else {
            self.key_complex(input)
        }
    }

    fn key_simple(&mut self, key: VirtualKeyCode) -> EventOut {
        match key {
            VirtualKeyCode::Escape => EventOut::ControlFlow(ControlFlow::Exit),
            VirtualKeyCode::T => {
                self.gpu.texture_on = !self.gpu.texture_on;
                EventOut::None
            }
            VirtualKeyCode::Y => {
                self.gpu.prev_texture();
                EventOut::None
            }
            VirtualKeyCode::U => {
                self.gpu.next_texture();
                EventOut::None
            }
            VirtualKeyCode::R => {
                self.setting.rotate = !self.setting.rotate;
                EventOut::None
            }
            _ => {
                eprintln!("no bind for {:?}", key);
                EventOut::None
            }
        }
    }

    fn key_complex(&self, input: KeyboardInput) -> EventOut {
        EventOut::None
    }

    // will handle better and more events later
    fn cursor(&mut self, position: &PhysicalPosition<f64>) -> EventOut {
        // println!("CursorMoved:\n\tposition:\t{:?}", position);
        let (w, h) = self.display.get_framebuffer_dimensions();
        self.setting.bg_color = Color {
            r: position.x as ColorPrecision / w as ColorPrecision,
            g: position.y as ColorPrecision / h as ColorPrecision,
            b: 0.0,
            a: 0.5,
        };
        EventOut::None
    }
}
