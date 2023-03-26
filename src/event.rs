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
    fn cursor(&mut self, position: &PhysicalPosition<f64>) -> bool {
        // println!("CursorMoved:\n\tposition:\t{:?}", position);
        self.render.bg_color = wgpu::Color {
            r: position.x as f64 / self.size.width as f64,
            g: position.y as f64 / self.size.height as f64,
            b: 0.0,
            a: 1.0,
        };
        true
    }
}
