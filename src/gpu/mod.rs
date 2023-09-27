mod load_texture;
mod render;

use crate::prelude::*;

use crate::{obj::ColorType, Color, Object, Vertex};

use load_texture::load_texture;

use ivec::IVec;

pub type IndexType = u16;

pub struct Gpu {
    pub program: glium::Program,
    pub object: IVec<(
        IVec<glium::VertexBuffer<Vertex>>,
        glium::index::IndexBuffer<IndexType>,
    )>,
    pub texture: IVec<glium::texture::SrgbTexture2d>,
    pub texture_on: bool,
}

impl Gpu {
    pub fn new(display: &glium::Display, object: &[Object]) -> Result<Self> {
        let mut obj_data: IVec<(
            IVec<glium::VertexBuffer<Vertex>>,
            glium::index::IndexBuffer<IndexType>,
        )> = IVec::default();

        for o in object {
            dbg!(&o.name);
            let mut v = Vec::new();
			for color_type in [ColorType::Random, ColorType::Selection(vec![
                    Color::new(0.1, 0.1, 0.1, 1.0),
                    Color::new(0.2, 0.2, 0.2, 1.0),
                    Color::new(0.3, 0.3, 0.3, 1.0),
                ]),
            ] {
				let mut vertex = o.vertex.clone();
				color_type.apply(&mut vertex);
				v.push(glium::VertexBuffer::new(display, &vertex)?);
			}
            let index_buffer = glium::index::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &o.triangulate(),
            )?;
            obj_data.vec.push((v, index_buffer));
            // todo!();
        }

        let shape = vec![
            Vertex {
                position: [0.0, 0.5, 0.0, 1.0],
                color: [1.0, 0.0, 0.0, 1.0],
                texture: [0.5, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
                texture: [1.0, 0.0],
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape)?;
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // tmp
        // obj_data.push((vertex_buffer, index_buffer));

        Ok(Self {
            program: glium::Program::from_source(
                display,
                include_str!("main.vert"),
                include_str!("main.frag"),
                None,
            )?,
            object: obj_data.into(),
            texture: load_texture(display).into(),
            texture_on: false,
            .into(),
        })
    }
}
