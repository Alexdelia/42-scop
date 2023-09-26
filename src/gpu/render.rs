use crate::env::Env;

use glium::Surface;

impl Env {
    pub fn render(&self, rotate_angle: f32) {
        let mut frame = self.display.draw();

        frame.clear_color(
            self.setting.bg_color.r,
            self.setting.bg_color.g,
            self.setting.bg_color.b,
            self.setting.bg_color.a,
        );

        let uniform = glium::uniform! {
            matrix: rotate_matrix(rotate_angle),
            tex: self.gpu.get_texture(),
            // texture_on: self.gpu.texture_on,
            texture_on: true,
        };

        frame
            .draw(
                // &self.gpu.vertex_buffer,
                // &self.gpu.index_buffer,
                &self.gpu.object[0].0,
                &self.gpu.object[0].1,
                &self.gpu.program,
                &uniform,
                &Default::default(),
            )
            .unwrap();

        frame.finish().unwrap();
    }
}

fn rotate_matrix(angle: f32) -> [[f32; 4]; 4] {
    let c = angle.cos();
    let s = angle.sin();
    [
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ]
}
