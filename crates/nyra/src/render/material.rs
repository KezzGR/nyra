pub struct Material {
    pub base_color: [f32; 4],
}

impl Material {
    pub const fn new(base_color: [f32; 4]) -> Self {
        Self { base_color }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new([0.72, 0.74, 0.8, 1.0])
    }
}
