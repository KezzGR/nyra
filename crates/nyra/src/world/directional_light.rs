use nyra_math::Vec3;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub intensity: f32,
}

impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            direction: Vec3::new(1.0, 3.0, 2.0),
            intensity: 0.9,
        }
    }
}
