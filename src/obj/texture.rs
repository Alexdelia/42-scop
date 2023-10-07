use crate::Vertex;

use super::{Bound, Face};

pub enum TextureType {
    Unit,
    Global,
}

impl TextureType {
    pub fn apply<'a, V, F>(&self, vertex: V, face: F)
    where
        V: IntoIterator<Item = &'a mut Vertex>,
        F: IntoIterator<Item = &'a Face>,
    {
        match self {
            TextureType::Unit => Self::unit(vertex, face),
            TextureType::Global => Self::global(vertex),
        }
    }

    fn unit<'a, V, F>(vertex: V, face: F)
    where
        V: IntoIterator<Item = &'a mut Vertex>,
        F: IntoIterator<Item = &'a Face>,
    {
        let mut vertex = vertex.into_iter().collect::<Vec<_>>();

        for f in face.into_iter() {
            let local_vertex = f
                .iter()
                .map(|eface| *vertex[eface.vertex])
                .collect::<Vec<_>>();

            let bound = Bound::new(&local_vertex);

            for v in vertex.iter_mut() {
                let [x, y, z, ..] = v.position;

                v.texture[0] = (x + z) / 2.0;
                v.texture[1] = y;
            }
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
