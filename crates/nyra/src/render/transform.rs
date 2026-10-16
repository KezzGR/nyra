use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: Quat::IDENTITY.to_array(),
            scale: [1.0, 1.0, 1.0],
        }
    }
}

impl Transform {
    pub fn new(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn rotate_from_axis_angle(&mut self, axis: Vec3, angle: f32) {
        self.rotation = Quat::from_axis_angle(axis.normalize(), angle).to_array();
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            Vec3::from_array(self.scale),
            Quat::from_array(self.rotation),
            Vec3::from_array(self.position),
        )
    }
}
