#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
