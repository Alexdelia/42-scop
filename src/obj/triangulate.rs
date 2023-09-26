use glium::IndexBuffer;

use crate::{gpu::IndexType, VertexPrecision};

use super::{EFace, Object, Vertex};

impl Object {
    pub fn triangulate(&self) -> Vec<IndexType> {
        let point = self
            .vertex
            .iter()
            .map(|v| [v.x(), v.y(), v.z(), v.w()])
            .collect::<Vec<_>>();

        let mut ret = Vec::new();

        for face in &self.face {
            let face = triangulate(&point, face);
            ret.extend(face);
        }

        ret
    }
}

type Point = [VertexPrecision; 4];

fn triangulate(point: &[Point], face: &[EFace]) -> Vec<IndexType> {
    let mut queue: Vec<IndexType> = face.iter().map(|f| f.vertex as IndexType).collect();

    if queue.len() == 3 {
        return queue;
    }

    let mut ret = Vec::with_capacity((queue.len() - 2) * 3);

    while queue.len() > 3 {
        let mut i = 0;

        while i < queue.len() {
            let a = queue[i];
            let b = queue[(i + 1) % queue.len()];
            let c = queue[(i + 2) % queue.len()];

            if is_ear(&queue, point, a, b, c) {
                ret.push(a);
                ret.push(b);
                ret.push(c);

                queue.remove((i + 1) % queue.len());

                break;
            }

            i += 1;
        }
    }

    ret.extend(queue);

    ret
}

fn is_ear(queue: &[IndexType], point: &[Point], a: IndexType, b: IndexType, c: IndexType) -> bool {
    let a = a as usize;
    let b = b as usize;
    let c = c as usize;

    for i in queue {
        let i = *i as usize;
        if i != a && i != b && i != c {
            if is_in_triangle(&point[i], &point[a], &point[b], &point[c]) {
                return false;
            }
        }
    }

    true
}

fn is_in_triangle(p: &Point, a: &Point, b: &Point, c: &Point) -> bool {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2], b[3] - a[3]];
    let bc = [c[0] - b[0], c[1] - b[1], c[2] - b[2], c[3] - b[3]];
    let ca = [a[0] - c[0], a[1] - c[1], a[2] - c[2], a[3] - c[3]];

    let ap = [p[0] - a[0], p[1] - a[1], p[2] - a[2], p[3] - a[3]];
    let bp = [p[0] - b[0], p[1] - b[1], p[2] - b[2], p[3] - b[3]];
    let cp = [p[0] - c[0], p[1] - c[1], p[2] - c[2], p[3] - c[3]];

    let abp = cross(&ab, &ap);
    let bcp = cross(&bc, &bp);
    let cap = cross(&ca, &cp);

    abp[0] * bcp[0] + abp[1] * bcp[1] + abp[2] * bcp[2] + abp[3] * bcp[3] >= 0.0
        && bcp[0] * cap[0] + bcp[1] * cap[1] + bcp[2] * cap[2] + bcp[3] * cap[3] >= 0.0
        && cap[0] * abp[0] + cap[1] * abp[1] + cap[2] * abp[2] + cap[3] * abp[3] >= 0.0
}

fn cross(a: &Point, b: &Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        0.0,
    ]
}
