mod vertex;
pub use vertex::{Vertex, VertexPrecision};
mod color;
pub use color::{Color, ColorPrecision};
mod material;
pub use material::Material;

// https://en.wikipedia.org/wiki/Wavefront_.obj_file
// http://web.cse.ohio-state.edu/~shen.94/581/Site/Lab3_files/Labhelp_Obj_parser.htm

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

impl Default for Object {
    fn default() -> Self {
        Self {
            name: String::new(),
            group: None,
            vertex: Vec::new(),
            texture: Vec::new(),
            normal: Vec::new(),
            face: Vec::new(),
            material: None,
            smooth: false,
        }
    }
}

impl Object {
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            name: file.into(),
            ..Default::default()
        }
    }

    // index buffer
    pub fn triangulation(&self) -> Vec<u32> {
        let mut result = Vec::new();

        for face in &self.face {
            for i in 1..face.len() - 1 {
                result.push(face[0].vertex as u32);
                result.push(face[i].vertex as u32);
                result.push(face[i + 1].vertex as u32);
            }
        }

        result
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

pub struct EFace {
    pub vertex: usize,
    pub texture: Option<usize>,
    pub normal: Option<usize>,
}
