use crate::{matrix::transformation::TranslationAmount, Matrix, Vertex};

use super::Point;

pub struct Bound {
    pub min: Point,
    pub max: Point,
}

impl Bound {
    pub fn new<'a, I>(vertex: I) -> Self
    where
        I: IntoIterator<Item = &'a Vertex>,
    {
        let mut bound = Self {
            min: Point {
                x: f32::MAX,
                y: f32::MAX,
                z: f32::MAX,
            },
            max: Point {
                x: f32::MIN,
                y: f32::MIN,
                z: f32::MIN,
            },
        };

        for v in vertex {
            bound.min.x = bound.min.x.min(v.x());
            bound.min.y = bound.min.y.min(v.y());
            bound.min.z = bound.min.z.min(v.z());
            bound.max.x = bound.max.x.max(v.x());
            bound.max.y = bound.max.y.max(v.y());
            bound.max.z = bound.max.z.max(v.z());
        }
        bound
    }

    pub fn matrix(self) -> Matrix {
        let matrix = Matrix::translation(TranslationAmount {
            x: -(self.min.x + self.max.x) / 2.0,
            y: -(self.min.y + self.max.y) / 2.0,
            z: -(self.min.z + self.max.z) / 2.0,
        });

        let scale = 2.0
            / (self.max.x - self.min.x)
                .max(self.max.y - self.min.y)
                .max(self.max.z - self.min.z);

        matrix
            * Matrix::scale(TranslationAmount {
                x: scale,
                y: scale,
                z: scale,
            })
    }
}
