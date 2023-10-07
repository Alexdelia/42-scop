pub mod r#type;

pub type ColorPrecision = f32;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: ColorPrecision,
    pub g: ColorPrecision,
    pub b: ColorPrecision,
    pub a: ColorPrecision,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Color {
    pub fn new(r: ColorPrecision, g: ColorPrecision, b: ColorPrecision, a: ColorPrecision) -> Self {
        Self { r, g, b, a }
    }

    pub fn hsva(
        h: ColorPrecision,
        s: ColorPrecision,
        v: ColorPrecision,
        a: ColorPrecision,
    ) -> Self {
        let i = (h * 6.0).floor() as i32;
        let f = h * 6.0 - i as ColorPrecision;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        match i % 6 {
            0 => Self::new(v, t, p, a),
            1 => Self::new(q, v, p, a),
            2 => Self::new(p, v, t, a),
            3 => Self::new(p, q, v, a),
            4 => Self::new(t, p, v, a),
            5 => Self::new(v, p, q, a),
            _ => unreachable!(),
        }
    }
}

impl From<Color> for [ColorPrecision; 4] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<Color> for [ColorPrecision; 3] {
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b]
    }
}
