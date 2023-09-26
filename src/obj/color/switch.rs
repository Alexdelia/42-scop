use rand::Rng;

use super::Color;
use crate::Vertex;

pub enum ColorType {
    Random,
    Selection(Vec<Color>),
}

impl ColorType {
    pub fn apply<'a, I>(&self, vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        match self {
            ColorType::Random => Self::random(vertex),
            ColorType::Selection(colors) => Self::selection(vertex, &colors),
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
}
