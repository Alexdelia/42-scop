mod vertex;
pub use vertex::Vertex;
mod color;
pub use color::Color;
mod material;
pub use material::Material;

// https://en.wikipedia.org/wiki/Wavefront_.obj_file
// http://web.cse.ohio-state.edu/~shen.94/581/Site/Lab3_files/Labhelp_Obj_parser.htm

pub struct Object {
    pub name: String,
    pub group: String,
    pub vertex: Vec<Vertex>,
    pub texture: Vec<VertexTexture>,
    pub normal: Vec<VertexNormal>,
    // pub parameter_space: Vec<Vertex>,
    pub face: Vec<Face>,
    pub material: Material, // usemtl not fully implemented
    pub smooth: bool,       // not implemented
}

pub struct VertexNormal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct VertexTexture {
    pub u: f32, // horizontal texture coordinate
    pub v: f32, // vertical texture coordinate
    pub w: f32, // depth
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

pub struct EFace {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
}
