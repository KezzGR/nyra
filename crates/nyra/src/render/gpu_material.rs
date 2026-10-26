use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
    BufferDescriptor, BufferUsages, Device,
};

use crate::MaterialUniform;

pub struct GpuMaterial {
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
}

impl GpuMaterial {
    pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Material Uniform Buffer"),
            size: size_of::<MaterialUniform>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Material Bind Group"),
            layout: layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            uniform_buffer,
            bind_group,
        }
    }
}
