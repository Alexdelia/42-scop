use crate::{matrix::transformation::TranslationAmount, Matrix, Vertex, VertexPrecision};

pub struct Bound {
    pub min: [VertexPrecision; 3],
    pub max: [VertexPrecision; 3],
}

impl Bound {
    pub fn new(vertex: &[Vertex]) -> Self {
        let mut bound = Self {
            min: [f32::MAX, f32::MAX, f32::MAX],
            max: [f32::MIN, f32::MIN, f32::MIN],
        };

        for v in vertex {
            bound.min[0] = bound.min[0].min(v.x());
            bound.min[1] = bound.min[1].min(v.y());
            bound.min[2] = bound.min[2].min(v.z());
            bound.max[0] = bound.max[0].max(v.x());
            bound.max[1] = bound.max[1].max(v.y());
            bound.max[2] = bound.max[2].max(v.z());
        }
        bound
    }

    pub fn matrix(self) -> Matrix {
        let matrix = Matrix::translation(TranslationAmount {
            x: -(self.min[0] + self.max[0]) / 2.0,
            y: -(self.min[1] + self.max[1]) / 2.0,
            z: -(self.min[2] + self.max[2]) / 2.0,
        });

        let scale = 2.0
            / (self.max[0] - self.min[0])
                .max(self.max[1] - self.min[1])
                .max(self.max[2] - self.min[2]);

        matrix
            * Matrix::scale(TranslationAmount {
                x: scale,
                y: scale,
                z: scale,
            })
    }
}
