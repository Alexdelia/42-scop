use crate::prelude::*;

use crate::{
    obj::{Bound, ColorType, TextureType},
    Color, Object,
};

use super::GpuObject;

use ivec::IVec;

fn color_option() -> Vec<ColorType> {
    vec![
        ColorType::Random,
        ColorType::Selection(vec![
            Color::new(0.1, 0.1, 0.1, 1.0),
            Color::new(0.2, 0.2, 0.2, 1.0),
            Color::new(0.3, 0.3, 0.3, 1.0),
        ]),
        ColorType::YGradient((
            Color::new(1.0, 0.0, 0.0, 1.0),
            Color::new(0.0, 0.0, 1.0, 1.0),
        )),
        ColorType::YGradient((
            Color::new(0.0, 1.0, 0.33, 0.0),
            Color::new(0.0, 1.0, 0.0, 1.0),
        )),
    ]
}

fn texture_option() -> Vec<TextureType> {
    vec![TextureType::Global, TextureType::Unit]
}

pub fn process_object(display: &glium::Display, object: &[Object]) -> Result<GpuObject> {
    let mut obj_data = IVec::new();

    for o in object {
        let mut v = Vec::new();
        let texture_option = texture_option();

        for (i, color_type) in color_option().iter().enumerate() {
            let mut vertex = o.vertex.clone();

            color_type.apply(&mut vertex);
            texture_option[i % texture_option.len()].apply(&mut vertex);

            v.push(glium::VertexBuffer::new(display, &vertex)?);
        }

        let index_buffer = glium::index::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &o.triangulate(),
        )?;

        let matrix = Bound::new(o.used_vertex()).matrix();

        obj_data.vec.push((v.into(), index_buffer, matrix));
    }

    Ok(obj_data)
}
