use crate::env::Env;

use glium::Surface;

impl Env {
    pub fn render(&self, t: f32) {
        let mut frame = self.display.draw();

        frame.clear_color(
            self.setting.bg_color.r,
            self.setting.bg_color.g,
            self.setting.bg_color.b,
            self.setting.bg_color.a,
        );

        let uniform = glium::uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: self.gpu.get_texture(),
            texture_on: self.gpu.texture_on,
        };

        frame
            .draw(
                &self.gpu.vertex_buffer,
                &self.gpu.index_buffer,
                &self.gpu.program,
                &uniform,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    }
}
