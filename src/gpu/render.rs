use crate::{ColorPrecision, Env, LoopData, Matrix};

use glium::{implement_uniform_block, uniforms::UniformBuffer, Surface};

#[repr(C)]
#[derive(Clone, Copy)]
struct ColorBuffer {
    colors: [[ColorPrecision; 4]; 5],
    len: u32,
}

implement_uniform_block!(ColorBuffer, colors, len);

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

        let mut model = self.gpu.object.get().2;
        model *= Matrix::rotation(data.rotation);
        model *= Matrix::translation(self.setting.translation);

        let color_buffer = UniformBuffer::immutable(
            &self.display,
            ColorBuffer {
                colors: self.setting.color_face.0.get().colors,
                len: self.setting.color_face.0.get().len,
            },
        )
        .unwrap();

        let uniform = glium::uniform! {
            perspective: Matrix::perspective(frame.get_dimensions()),
            model: model,

            ColorBuffer: &color_buffer,
            use_color_buffer: self.setting.color_face.1,

            tex: self.gpu.texture.get(),
            textured: self.setting.textured,

            enlighten: self.setting.enlighten,
            light: [-1.0, 0.4, -0.5f32],
            strength: 0.2f32,
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
