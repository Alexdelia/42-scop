#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub texture: [f32; 2],
}

glium::implement_vertex!(Vertex, position, color, texture);

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [0.0, 0.0, 0.0],
            texture: [0.0, 0.0],
        }
    }
}

impl Vertex {
    pub fn x(&self) -> f32 {
        self.position[0]
    }

    pub fn y(&self) -> f32 {
        self.position[1]
    }

    pub fn z(&self) -> f32 {
        self.position[2]
    }

    pub fn r(&self) -> f32 {
        self.color[0]
    }

    pub fn g(&self) -> f32 {
        self.color[1]
    }

    pub fn b(&self) -> f32 {
        self.color[2]
    }
}
