use rand::Rng;

use super::Color;
use crate::{Vertex, VertexPrecision};

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
            ColorType::Selection(colors) => Self::selection(vertex, colors),
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
        for (i, v) in vertex.into_iter().enumerate() {
            v.color = colors[i % colors.len()].clone().into();
        }
    }

    fn y_gradient<'a, I>(vertex: I, start: &Color, end: &Color)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        let min = VertexPrecision::MAX;
        let max = VertexPrecision::MIN;

        let v = vertex.into_iter().collect::<Vec<_>>();

        let (min, max) = v.iter().fold((min, max), |(min, max), v| {
            let [_, y, ..] = v.position;

            (min.min(y), max.max(y))
        });

        for v in v.into_iter() {
            let [_, y, ..] = v.position;
            let t = (y - min) / (max - min);

            v.color[0] = start.r + (end.r - start.r) * t;
            v.color[1] = start.g + (end.g - start.g) * t;
            v.color[2] = start.b + (end.b - start.b) * t;
            v.color[3] = start.a + (end.a - start.a) * t;
        }
    }
}
