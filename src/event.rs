use crate::env::Env;

use glium::glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

pub enum EventOut {
    None,
    ControlFlow(ControlFlow),
}

impl Env {
    pub fn event(&self, event: Event<()>) -> EventOut {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => return EventOut::ControlFlow(ControlFlow::Exit),
                WindowEvent::KeyboardInput { input, .. } => self.key(input),
                _ => EventOut::None,
            },
            _ => EventOut::None,
        }
    }

    fn key(&self, input: KeyboardInput) -> EventOut {
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

    fn key_simple(&self, key: VirtualKeyCode) -> EventOut {
        match key {
            VirtualKeyCode::Escape => EventOut::ControlFlow(ControlFlow::Exit),
            _ => {
                eprintln!("no bind for {:?}", key);
                EventOut::None
            }
        }
    }

    fn key_complex(&self, input: KeyboardInput) -> EventOut {
        EventOut::None
    }
}
