mod env;
mod event;
mod gpu;
mod setting;
pub use gpu::Color;

use env::Env;
use event::EventOut;

use yahmrslib::hmerr::Result;

use glium::glutin;
use glium::Surface;

fn main() -> Result<()> {
    println!("Hello, world!");

    event_loop()
}

pub fn event_loop() -> Result<()> {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let mut env = Env::new(&event_loop)?;

    let mut t: f32 = -0.5;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = env.setting.fps();

        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut frame = env.display.draw();
        frame.clear_color(
            env.setting.bg_color.r,
            env.setting.bg_color.g,
            env.setting.bg_color.b,
            env.setting.bg_color.a,
        );
        let uniform = glium::uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        frame
            .draw(
                &env.gpu.vertex_buffer,
                &env.gpu.index_buffer,
                &env.gpu.program,
                &uniform,
                &Default::default(),
            )
            .unwrap();
        frame.finish().unwrap();

        if let EventOut::ControlFlow(cf) = env.event(event) {
            *control_flow = cf;
            if cf == glutin::event_loop::ControlFlow::Exit {
                return;
            }
        }
    });
}
