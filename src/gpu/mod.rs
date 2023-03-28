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
    texture: Option<glium::texture::SrgbTexture2d>,
}

impl Gpu {
    pub fn new(display: &glium::Display) -> Result<Self> {
        let shape = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0],
                texture: [0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
                texture: [0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0],
                texture: [1.0, 0.0],
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
            texture: None,
        })
    }

    pub fn get_texture(&self) -> Option<&glium::texture::SrgbTexture2d> {
        self.texture.as_ref()
    }

    pub fn set_texture(
        &mut self,
        display: &glium::Display,
        texture: Option<&std::path::Path>,
    ) -> Result<()> {
        let Some(path) = texture else {
			self.texture = None;
			return Ok(());
		};

        let image = image::open(path)
            .map_err(|e| {
                eprintln!("failed to open texture path '{}'\n{e}", path.display());
                e
            })?
            .into_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        self.texture = Some(
            glium::texture::SrgbTexture2d::new(display, image).map_err(|e| {
                eprintln!(
                    "failed to create texture from image '{}'\n{e}",
                    path.display(),
                );
                e
            })?,
        );
        Ok(())

        /*
        use std::io::Cursor;
        let image = image::load(
            Cursor::new(&include_bytes!("/path/to/image.png")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        */
    }
}
