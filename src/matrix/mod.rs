pub mod transformation;

use crate::VertexPrecision;

use std::ops::{Mul, MulAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix(pub [[VertexPrecision; 4]; 4]);

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut m = [[0.0; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    m[a][b] += self.0[a][c] * rhs.0[c][b];
                }
            }
        }
        Matrix(m)
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = *self * rhs;
    }
}

impl Matrix {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}
