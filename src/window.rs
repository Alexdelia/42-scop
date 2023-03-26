// use crate::setting::FPS;
use crate::env::Env;

use yahmrslib::hmerr::Result;

use glium::glutin;

// use std::time::{Duration, Instant};

pub fn run() -> Result<()> {
    let mut env = Env::new();

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame());
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        env.event()?;
        // need to return something special and handle control flow
        // *control_flow = glutin::event_loop::ControlFlow::Exit;

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}

// fn next_frame() -> Instant {
//     Instant::now() + Duration::from_nanos(1_000_000_000 / FPS as u64)
// }
