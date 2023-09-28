#![allow(unused)] // TODO: remove

mod env;
mod event;
mod gpu;
mod matrix;
mod obj;
mod parse;
mod prelude;
mod setting;

use prelude::*;

pub use obj::{Color, ColorPrecision, Object, Vertex, VertexPrecision};

use matrix::{transformation::RotationAmount, Matrix};

use env::Env;
use event::EventOut;

use glium::glutin;

fn main() -> Result<()> {
    let object = parse::parse()?;

    event_loop(object)
}

#[derive(Default)]
pub struct LoopData {
    t: f32,
    rotation: RotationAmount,
}

pub fn event_loop(object: Vec<Object>) -> Result<()> {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut env = Env::new(&event_loop, object)?;
    let mut loop_data = LoopData::default();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = env.setting.fps();

        loop_data.update(&env);

        env.render(&loop_data);

        if let EventOut::ControlFlow(cf) = env.event(event) {
            *control_flow = cf;
            if cf == glutin::event_loop::ControlFlow::Exit {
                return;
            }
        }
    });
}

impl LoopData {
    fn update(&mut self, env: &Env) {
        self.t = (self.t + env.setting.speed) % 1.0;

        env.setting.rotation(&mut self.rotation);
    }
}
