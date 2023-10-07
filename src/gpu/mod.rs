mod load_texture;
mod process_object;
mod render;

use crate::prelude::*;

use crate::{Matrix, Object, Vertex};

use load_texture::load_texture;

use ivec::IVec;

pub type IndexType = u16;

type GpuObject = IVec<(
    IVec<glium::VertexBuffer<Vertex>>,
    glium::index::IndexBuffer<IndexType>,
    Matrix,
)>;

pub struct Gpu {
    pub program: glium::Program,
    pub object: GpuObject,
    pub texture: IVec<glium::texture::SrgbTexture2d>,
    pub texture_on: bool,
}

impl Gpu {
    pub fn new(display: &glium::Display, object: &[Object]) -> Result<Self> {
        let obj_data = process_object::process_object(display, object)?;

        Ok(Self {
            program: glium::Program::from_source(
                display,
                include_str!("main.vert"),
                include_str!("main.frag"),
                None,
            )?,
            object: obj_data,
            texture: load_texture(display).into(),
            texture_on: false,
        })
    }
}
