use crate::VertexPrecision;

use super::Matrix;

#[derive(Default, Clone, Copy)]
pub struct TranslationAmount {
    pub x: VertexPrecision,
    pub y: VertexPrecision,
    pub z: VertexPrecision,
}

#[derive(Default, Clone, Copy)]
pub struct RotationAmount {
    pub x: VertexPrecision,
    pub y: VertexPrecision,
    pub z: VertexPrecision,
}

impl Matrix {
    pub fn translation(t: TranslationAmount) -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [t.x, t.y, t.z, 1.0],
        ])
    }

    /// rotate matrix
    ///
    /// matrix x rotation:
    /// ```no_run
    /// [
    ///     [1.0, 0.0, 0.0, 0.0],
    ///     [0.0, cos, -sin, 0.0],
    ///     [0.0, sin, cos, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ]
    /// ```
    ///
    /// matrix y rotation:
    /// ```no_run
    /// [
    ///     [cos, 0.0, sin, 0.0],
    ///     [0.0, 1.0, 0.0, 0.0],
    ///     [-sin, 0.0, cos, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ]
    /// ```
    ///
    /// matrix z rotation:
    /// ```no_run
    /// [
    ///     [cos, -sin, 0.0, 0.0],
    ///     [sin, cos, 0.0, 0.0],
    ///     [0.0, 0.0, 1.0, 0.0],
    ///     [0.0, 0.0, 0.0, 1.0],
    /// ]
    /// ```
    pub fn rotation(r: RotationAmount) -> Self {
        let (sin_x, cos_x) = r.x.sin_cos();
        let (sin_y, cos_y) = r.y.sin_cos();
        let (sin_z, cos_z) = r.z.sin_cos();

        Self([
            [
                cos_z * cos_y,
                cos_z * sin_y * sin_x - sin_z * cos_x,
                cos_z * sin_y * cos_x + sin_z * sin_x,
                0.0,
            ],
            [
                sin_z * cos_y,
                sin_z * sin_y * sin_x + cos_z * cos_x,
                sin_z * sin_y * cos_x - cos_z * sin_x,
                0.0,
            ],
            [-sin_y, cos_y * sin_x, cos_y * cos_x, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn perspective(dimensions: (u32, u32)) -> Self {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;
        let fov: f32 = std::f32::consts::PI / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        Self([
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ])
    }
}
