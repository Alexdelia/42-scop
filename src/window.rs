// use crate::setting::FPS;
use crate::env::Env;
use crate::event::EventOut;

use yahmrslib::hmerr::Result;

use glium::glutin;

// use std::time::{Duration, Instant};

pub fn run() -> Result<()> {
    let mut env = Env::new();

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame());
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        match env.event(event) {
            EventOut::None => (),
            EventOut::ControlFlow(cf) => {
                *control_flow = cf;
                if cf == glutin::event_loop::ControlFlow::Exit {
                    return;
                }
            }
        }
    });
}

// fn next_frame() -> Instant {
//     Instant::now() + Duration::from_nanos(1_000_000_000 / FPS as u64)
// }
