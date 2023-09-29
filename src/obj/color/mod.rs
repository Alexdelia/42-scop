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
