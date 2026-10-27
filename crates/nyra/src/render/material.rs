use crate::TextureID;

pub struct Material {
    pub base_color: [f32; 4],
    pub texture_id: Option<TextureID>,
}

impl Material {
    pub const fn new(base_color: [f32; 4]) -> Self {
        Self {
            base_color,
            texture_id: None,
        }
    }

    pub fn with_texture(mut self, texture_id: TextureID) -> Self {
        self.texture_id = Some(texture_id);
        self
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new([0.72, 0.74, 0.8, 1.0])
    }
}
