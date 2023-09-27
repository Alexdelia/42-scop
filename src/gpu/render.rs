use crate::env::Env;

use glium::{DrawParameters, Surface};

impl Env {
    pub fn render(&self, rotate_angle: f32) {
        let mut frame = self.display.draw();

        frame.clear_color_and_depth(
            (
                self.setting.bg_color.r,
                self.setting.bg_color.g,
                self.setting.bg_color.b,
                self.setting.bg_color.a,
            ),
            1.0,
        );

        let mut matrix = rotate_matrix(rotate_angle);
        zoom_matrix(&mut matrix, self.setting.zoom_amount);

        let uniform = glium::uniform! {
            matrix: matrix,
            tex: self.gpu.texture.get(),
            texture_on: self.gpu.texture_on,
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };
        // let params = DrawParameters::default();

        let object = self.gpu.object.get();

        frame
            .draw(&object.0, &object.1, &self.gpu.program, &uniform, &params)
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

fn zoom_matrix(matrix: &mut [[f32; 4]; 4], amount: f32) {
    matrix[3][3] *= amount;
}
