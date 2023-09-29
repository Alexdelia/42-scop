mod vertex;
pub use vertex::{Vertex, VertexPrecision};
mod color;
pub use color::{r#type::ColorType, Color, ColorPrecision};
mod material;
pub use material::Material;
mod triangulate;

// https://en.wikipedia.org/wiki/Wavefront_.obj_file
// http://web.cse.ohio-state.edu/~shen.94/581/Site/Lab3_files/Labhelp_Obj_parser.htm

#[derive(Default)]
pub struct Object {
    pub name: String,
    pub group: Option<String>,
    pub vertex: Vec<Vertex>,
    pub texture: Vec<VertexTexture>,
    pub normal: Vec<VertexNormal>,
    // pub parameter_space: Vec<Vertex>,
    pub face: Vec<Face>,
    pub material: Option<Material>, // usemtl not fully implemented
    pub smooth: bool,               // not implemented
}

impl Object {
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            name: file.into(),
            ..Default::default()
        }
    }
}

pub struct VertexNormal {
    pub x: VertexPrecision,
    pub y: VertexPrecision,
    pub z: VertexPrecision,
}

pub struct VertexTexture {
    pub u: VertexPrecision, // horizontal texture coordinate
    pub v: VertexPrecision, // vertical texture coordinate
    pub w: VertexPrecision, // depth
}

impl Default for VertexTexture {
    fn default() -> Self {
        Self {
            u: 0.0,
            v: 0.0,
            w: 0.0,
        }
    }
}

pub type Face = Vec<EFace>; // 3 or more elements

#[derive(Clone, Copy)]
pub struct EFace {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
}
