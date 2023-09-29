use crate::Vertex;

use super::Bound;

pub enum TextureType {
    Local,
    Global,
}

impl TextureType {
    pub fn apply<'a, I>(&self, vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        match self {
            TextureType::Local => Self::local(vertex),
            TextureType::Global => Self::global(vertex),
        }
    }

    fn local<'a, I>(vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        for v in vertex {
            let [x, y, z, ..] = v.position;

            let bound = Bound::new(&[*v]);

            v.texture[0] = (x - bound.min.x) / (bound.max.x - bound.min.x);
            v.texture[1] = (y - bound.min.y) / (bound.max.y - bound.min.y);
        }
    }

    fn global<'a, I>(vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        let vertex = vertex.into_iter().collect::<Vec<_>>();

        let bound = Bound::new(&vertex.iter().map(|v| **v).collect::<Vec<_>>());

        for v in vertex.into_iter() {
            let [x, y, z, ..] = v.position;

            v.texture[0] = (x - bound.min.x) / (bound.max.x - bound.min.x);
            v.texture[1] = (y - bound.min.y) / (bound.max.y - bound.min.y);
        }
    }
}
