use crate::state::State;

use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

impl State {
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(key),
                        ..
                    },
                ..
            } => self.key(*key),
            WindowEvent::CursorMoved { position, .. } => self.cursor(position),
            _ => false,
        }
    }

    fn key(&self, key: VirtualKeyCode) -> bool {
        match key {
            VirtualKeyCode::A => println!("A"),
            _ => {
                println!("no bind for {:?}", key);
                return false;
            }
        };
        true
    }

    // will handle better and more events later
    fn cursor(&self, position: &PhysicalPosition<f64>) -> bool {
        println!("CursorMoved:\n\tposition:\t{:?}", position);
        true
    }
}
