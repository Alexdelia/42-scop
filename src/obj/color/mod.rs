pub mod switch;

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

impl Into<[ColorPrecision; 4]> for Color {
    fn into(self) -> [ColorPrecision; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<[ColorPrecision; 3]> for Color {
    fn into(self) -> [ColorPrecision; 3] {
        [self.r, self.g, self.b]
    }
}

impl
    Into<(
        ColorPrecision,
        ColorPrecision,
        ColorPrecision,
        ColorPrecision,
    )> for Color
{
    fn into(
        self,
    ) -> (
        ColorPrecision,
        ColorPrecision,
        ColorPrecision,
        ColorPrecision,
    ) {
        (self.r, self.g, self.b, self.a)
    }
}

impl Into<(ColorPrecision, ColorPrecision, ColorPrecision)> for Color {
    fn into(self) -> (ColorPrecision, ColorPrecision, ColorPrecision) {
        (self.r, self.g, self.b)
    }
}
