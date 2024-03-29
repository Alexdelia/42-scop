use crate::Axis;

use super::Object;

pub type VertexPrecision = f32;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [VertexPrecision; 4],
    pub color: [VertexPrecision; 4],
    pub texture: [VertexPrecision; 2],
    pub normal: [VertexPrecision; 3],
}

glium::implement_vertex!(Vertex, position, color, texture, normal);

pub struct Point {
    pub x: VertexPrecision,
    pub y: VertexPrecision,
    pub z: VertexPrecision,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0, 1.0],
            color: [0.0, 0.0, 0.0, 1.0],
            texture: [0.0, 0.0],
            normal: [0.0, 0.0, 0.0],
        }
    }
}

impl Object {
    pub fn used_vertex(&self) -> Vec<&Vertex> {
        self.face
            .iter()
            .flat_map(|f| f.iter().map(|ef| &self.vertex[ef.vertex]))
            .collect()
    }
}

impl Vertex {
    pub fn x(&self) -> VertexPrecision {
        self.position[0]
    }

    pub fn y(&self) -> VertexPrecision {
        self.position[1]
    }

    pub fn z(&self) -> VertexPrecision {
        self.position[2]
    }

    pub fn w(&self) -> VertexPrecision {
        self.position[3]
    }

    pub fn r(&self) -> VertexPrecision {
        self.color[0]
    }

    pub fn g(&self) -> VertexPrecision {
        self.color[1]
    }

    pub fn b(&self) -> VertexPrecision {
        self.color[2]
    }

    pub fn a(&self) -> VertexPrecision {
        self.color[3]
    }

    pub fn axis(&self, axis: Axis) -> VertexPrecision {
        match axis {
            Axis::X => self.x(),
            Axis::Y => self.y(),
            Axis::Z => self.z(),
        }
    }
}

impl Point {
    pub fn new(x: VertexPrecision, y: VertexPrecision, z: VertexPrecision) -> Self {
        Self { x, y, z }
    }
}
