use crate::setting::FPS;

use glium::glutin;

use std::time::{Duration, Instant};

pub fn run() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    event_loop.run(move |ev, _, control_flow| {
        // *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame());
        *control_flow = glutin::event_loop::ControlFlow::Poll;
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
