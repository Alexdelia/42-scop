use crate::Vertex;

use super::Bound;

pub enum TextureType {
    Unit,
    Global,
}

impl TextureType {
    pub fn apply<'a, I>(&self, vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        match self {
            TextureType::Unit => Self::unit(vertex),
            TextureType::Global => Self::global(vertex),
        }
    }

    fn unit<'a, I>(vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        for v in vertex {
            let [x, y, z, ..] = v.position;

            v.texture[0] = (x + z) / 2.0;
            v.texture[1] = y;
        }
    }

    fn global<'a, I>(vertex: I)
    where
        I: IntoIterator<Item = &'a mut Vertex>,
    {
        let vertex = vertex.into_iter().collect::<Vec<_>>();

        let bound = Bound::new(&vertex.iter().map(|v| **v).collect::<Vec<_>>());

        let front_side: bool = bound.max.x - bound.min.x > bound.max.z - bound.min.z;

        for v in vertex.into_iter() {
            let [x, y, z, ..] = v.position;

            if front_side {
                v.texture[0] = (x - bound.min.x) / (bound.max.x - bound.min.x);
            } else {
                v.texture[0] = (z - bound.min.z) / (bound.max.z - bound.min.z);
            }

            v.texture[1] = (y - bound.min.y) / (bound.max.y - bound.min.y);
        }
    }
}
