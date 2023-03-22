use crate::state::State;

use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
    window::Theme,
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
            } => match key {
                VirtualKeyCode::A => {
                    println!("Theme was: {:?}", self.window().theme());
                    self.window().set_theme(None);
                    true
                }
                VirtualKeyCode::L => {
                    println!("Theme was: {:?}", self.window().theme());
                    self.window().set_theme(Some(Theme::Light));
                    true
                }
                VirtualKeyCode::D => {
                    println!("Theme was: {:?}", self.window().theme());
                    self.window().set_theme(Some(Theme::Dark));
                    true
                }
                _ => {
                    println!("no bind for {:?}", event);
                    false
                }
            },
            _ => false,
        }
    }
}
