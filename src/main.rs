#![allow(unused)]

mod env;
mod event;
mod gpu;
mod helper;
mod matrix;
mod obj;
mod parse;
mod prelude;
mod setting;

use prelude::*;

pub use obj::{Color, ColorPrecision, Object, Vertex, VertexPrecision};

pub use matrix::{transformation::RotationAmount, Matrix};

use env::Env;
use event::EventOut;

use glium::glutin;

fn main() -> Result<()> {
    #[cfg(not(debug_assertions))]
    helper::header();

    let object = parse::parse()?;

    event_loop(object)
}

#[derive(Default)]
pub struct LoopData {
    t: f32,
    rotation: RotationAmount,
}

pub fn event_loop(object: Vec<Object>) -> Result<()> {
    let event_loop = glutin::event_loop::EventLoop::new();
    let mut env = Env::new(&event_loop, object)?;

    helper::help();

    println!();
    env.setting.print_fps();

    let mut loop_data = LoopData::default();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = env.setting.control_flow();

        loop_data.update(&env);

        env.render(&loop_data);

        if let EventOut::ControlFlow(cf) = env.event(event) {
            *control_flow = cf;
        }
    });
}

impl LoopData {
    fn update(&mut self, env: &Env) {
        self.t = (self.t + env.setting.speed.get()) % 1.0;

        env.setting.rotation(&mut self.rotation);
    }
}
