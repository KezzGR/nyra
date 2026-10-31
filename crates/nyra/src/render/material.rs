use nyra_math::Vec2;

use crate::TextureID;

pub struct Material {
    pub base_color: [f32; 4],
    pub texture_id: Option<TextureID>,
    pub uv_scale: Vec2,
    pub uv_offset: Vec2,
}

impl Material {
    pub const fn new(base_color: [f32; 4]) -> Self {
        Self {
            base_color,
            texture_id: None,
            uv_scale: Vec2::ONE,
            uv_offset: Vec2::ZERO,
        }
    }

    pub fn with_texture(mut self, texture_id: TextureID) -> Self {
        self.texture_id = Some(texture_id);
        self
    }

    pub fn with_uv_scale(mut self, scale: Vec2) -> Self {
        self.uv_scale = scale;
        self
    }

    pub fn with_uv_offset(mut self, offset: Vec2) -> Self {
        self.uv_offset = offset;
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new([0.72, 0.74, 0.8, 1.0])
    }
}
