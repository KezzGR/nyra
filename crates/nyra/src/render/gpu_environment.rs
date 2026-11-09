use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress,
    BufferDescriptor, BufferUsages, Device,
};

use crate::EnvironmentUniform;

pub struct GpuEnvironment {
    pub uniform_buffer: Buffer,
    pub bind_group: BindGroup,
}

impl GpuEnvironment {
    pub fn new(device: &Device, layout: &BindGroupLayout) -> Self {
        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Environment Uniform Buffer"),
            size: size_of::<EnvironmentUniform>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Environment Bind Group"),
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
