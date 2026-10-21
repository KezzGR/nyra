use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
    BufferUsages, Device, wgt::BufferDescriptor,
};

use crate::render::TransformUniform;

pub struct GpuThing {
    pub uniform_buffer: Buffer,
    pub uniform_bind_group: BindGroup,
}

impl GpuThing {
    pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: size_of::<TransformUniform>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform_buffer,
            uniform_bind_group,
        }
    }
}
