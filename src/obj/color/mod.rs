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
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        let mut i = (h * 6.0).floor() as i32;
        let mut f = h * 6.0 - i as ColorPrecision;
        let mut p = v * (1.0 - s);
        let mut q = v * (1.0 - f * s);
        let mut t = v * (1.0 - (1.0 - f) * s);
        match i % 6 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }
            _ => unreachable!(),
        }
        Self::new(r, g, b, a)
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
