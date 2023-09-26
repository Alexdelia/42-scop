mod load_texture;
mod render;

use crate::prelude::*;

use crate::{Object, Vertex};

use load_texture::load_texture;

pub enum TextureSource {
    Original,
    Modified,
    None,
}

pub struct Gpu {
    pub program: glium::Program,
    pub object: Vec<(glium::VertexBuffer<Vertex>, glium::index::NoIndices)>,
    pub external_texture: (Vec<glium::texture::SrgbTexture2d>, usize),
    pub texture_source: TextureSource,
}

impl Gpu {
    pub fn new(display: &glium::Display, object: &[Object]) -> Result<Self> {
        let mut obj_data = Vec::new();

        for o in object {
            let vertex_buffer = glium::VertexBuffer::new(display, &o.vertex)?;
            let index_buffer = glium::index::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &o.triangulation(),
            )?;
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
        obj_data.push((vertex_buffer, index_buffer));

        Ok(Self {
            program: glium::Program::from_source(
                display,
                include_str!("main.vert"),
                include_str!("main.frag"),
                None,
            )?,
            object: obj_data,
            external_texture: (load_texture(display), 0),
            texture_source: TextureSource::Original,
        })
    }

    pub fn get_texture(&self) -> &glium::texture::SrgbTexture2d {
        &self.external_texture.0[self.external_texture.1]
    }

    pub fn next_texture(&mut self) {
        self.external_texture.1 = (self.external_texture.1 + 1) % self.external_texture.0.len();
    }

    pub fn prev_texture(&mut self) {
        self.external_texture.1 = if self.external_texture.1 == 0 {
            self.external_texture.0.len() - 1
        } else {
            self.external_texture.1 - 1
        };
    }
}
