use crate::{env::Env, matrix::Matrix, LoopData};

use glium::Surface;

impl Env {
    pub fn render(&self, data: &LoopData) {
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

        let mut matrix = self.gpu.object.get().2;
        matrix *= Matrix::rotation(data.rotation);
        matrix *= Matrix::translation(self.setting.translation);

        let uniform = glium::uniform! {
            matrix: matrix.0,
            perspective: Matrix::perspective(frame.get_dimensions()).0,
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

        let object = self.gpu.object.get();

        frame
            .draw(
                object.0.get(),
                &object.1,
                &self.gpu.program,
                &uniform,
                &params,
            )
            .unwrap();

        frame.finish().unwrap();
    }
}
