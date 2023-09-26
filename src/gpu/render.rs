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

        let matrix = rotate_matrix(rotate_angle);
        let matrix = zoom_out_matrix(matrix, self.setting.zoom_amount);

        let uniform = glium::uniform! {
            matrix: matrix,
            tex: self.gpu.get_texture(),
            // texture_on: self.gpu.texture_on,
            texture_on: true,
        };

        let object = self.gpu.get_object();

        frame
            .draw(
                &object.0,
                &object.1,
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

fn zoom_out_matrix(matrix: [[f32; 4]; 4], amount: f32) -> [[f32; 4]; 4] {
    [
        [
            matrix[0][0] * amount,
            matrix[0][1],
            matrix[0][2],
            matrix[0][3],
        ],
        [
            matrix[1][0],
            matrix[1][1] * amount,
            matrix[1][2],
            matrix[1][3],
        ],
        [
            matrix[2][0],
            matrix[2][1],
            matrix[2][2] * amount,
            matrix[2][3],
        ],
        [
            matrix[3][0],
            matrix[3][1],
            matrix[3][2],
            matrix[3][3] * amount,
        ],
    ]
}
