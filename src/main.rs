mod env;
mod event;
mod render;
mod setting;
mod window;

use env::Env;
use event::EventOut;

// use yahmrslib::hmerr::Result;

use glium::glutin;

// use std::time::{Duration, Instant};

fn main() {
    println!("Hello, world!");

    event_loop()
}

pub fn event_loop() {
    let mut env = Env::new();

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = env.cf_fps();

        if let EventOut::ControlFlow(cf) = env.event(event) {
            *control_flow = cf;
            if cf == glutin::event_loop::ControlFlow::Exit {
                return;
            }
        }
    });
}

// fn next_frame() -> Instant {
//     Instant::now() + Duration::from_nanos(1_000_000_000 / FPS as u64)
// }
