use crate::Vertex;

use super::{Bound, Face};

pub enum TextureType {
    Local,
    Global,
}

impl TextureType {
    pub fn apply<'a, V, F>(&self, vertex: V, face: F)
    where
        V: IntoIterator<Item = &'a mut Vertex>,
        F: IntoIterator<Item = &'a Face>,
    {
        match self {
            TextureType::Local => Self::local(vertex, face),
            TextureType::Global => Self::global(vertex),
        }
    }

    fn local<'a, V, F>(vertex: V, face: F)
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

                // most top left vertex has texture = (0, 0)
                // most bottom right vertex has texture = (1, 1)
                v.texture[0] = (x - bound.min.x) / (bound.max.x - bound.min.x);
                v.texture[1] = (y - bound.min.y) / (bound.max.y - bound.min.y);
            }
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
