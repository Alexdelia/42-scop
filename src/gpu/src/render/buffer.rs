type BufferSize = u32;

pub struct Buffer {
    buffer: wgpu::Buffer,
    size: BufferSize,
}
