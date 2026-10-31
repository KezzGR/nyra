use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TransformUniform {
    pub model: [[f32; 4]; 4],
    pub projection_view: [[f32; 4]; 4],
}

impl TransformUniform {
    pub fn new(model: [[f32; 4]; 4], projection_view: [[f32; 4]; 4]) -> Self {
        Self {
            model,
            projection_view,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct MaterialUniform {
    base_color: [f32; 4],
    uv_scale: [f32; 2],
    uv_offset: [f32; 2],
}

impl MaterialUniform {
    pub fn new(base_color: [f32; 4], uv_scale: [f32; 2], uv_offset: [f32; 2]) -> Self {
        Self {
            base_color,
            uv_scale,
            uv_offset,
        }
    }
}
