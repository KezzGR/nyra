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
