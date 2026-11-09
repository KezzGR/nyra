use crate::DirectionalLight;

pub struct Environment {
    pub sun: DirectionalLight,
    pub ambient_intensity: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            sun: DirectionalLight::default(),
            ambient_intensity: 0.1,
        }
    }
}
