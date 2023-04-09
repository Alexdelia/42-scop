#![allow(unused)] // TODO: remove

mod env;
mod event;
mod gpu;
mod obj;
mod parse;
mod prelude;
mod setting;

use prelude::*;

pub use obj::{Color, Object, Vertex};

use env::Env;
use event::EventOut;

use glium::glutin;

fn main() -> Result<()> {
    let obj = parse::parse()?;

    event_loop()
}

pub fn event_loop() -> Result<()> {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut env = Env::new(&event_loop)?;

    const BASE: f32 = -180.0;
    const SHIFT: f32 = 0.001;
    let mut t: f32 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = env.setting.fps();

        if env.setting.rotate {
            t += SHIFT;
            if t > -BASE {
                t = BASE;
            }
        }

        env.render(t);

        if let EventOut::ControlFlow(cf) = env.event(event) {
            *control_flow = cf;
            if cf == glutin::event_loop::ControlFlow::Exit {
                return;
            }
        }
    });
}
