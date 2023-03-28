mod color;
pub use color::Color;
mod render;
mod vertex;
pub use vertex::Vertex;

use yahmrslib::hmerr::Result;

pub struct Gpu {
    pub program: glium::Program,
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub index_buffer: glium::index::NoIndices,
}

impl Gpu {
    pub fn new(display: &glium::Display) -> Result<Self> {
        let shape = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(display, &shape)?;
        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        Ok(Self {
            vertex_buffer,
            index_buffer,
            program: glium::Program::from_source(
                display,
                include_str!("main.vert"),
                include_str!("main.frag"),
                None,
            )?,
        })
    }
}
