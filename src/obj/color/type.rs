use rand::Rng;

use super::Color;
use crate::Vertex;

pub enum ColorType {
    Random,
    Selection(Vec<Color>),
    YGradient((Color, Color)),
}

impl ColorType {
    pub fn apply<'a, I>(&self, vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        match self {
            ColorType::Random => Self::random(vertex),
            ColorType::Selection(colors) => Self::selection(vertex, &colors),
            ColorType::YGradient((start, end)) => Self::y_gradient(vertex, start, end),
        }
    }

    fn random<'a, I>(vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        let mut rng = rand::thread_rng();

        for v in vertex {
            v.color[0] = rng.gen();
            v.color[1] = rng.gen();
            v.color[2] = rng.gen();
        }
    }

    fn selection<'a, I>(vertex: I, colors: &[Color])
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        let mut i = 0;

        for v in vertex {
            v.color = colors[i % colors.len()].clone().into();
            i += 1;
        }
    }

    /// gradient from color start at x = 1.0 to color end at x = -1.0
    fn y_gradient<'a, I>(vertex: I, start: &Color, end: &Color)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        for v in vertex {
            let [_, y, ..] = v.position;
            let y = y / 2.0 + 0.5;

            let r = start.r * y + end.r * (1.0 - y);
            let g = start.g * y + end.g * (1.0 - y);
            let b = start.b * y + end.b * (1.0 - y);
            let a = start.a * y + end.a * (1.0 - y);

            v.color = [r, g, b, a];
        }
    }
}
