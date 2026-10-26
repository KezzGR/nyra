use nyra_math::{Mat4, Quat, Vec3};

pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn translate(&mut self, position: Vec3) {
        self.position += position;
    }

    pub fn translate_local(&mut self, position: Vec3) {
        self.position += self.rotation * position;
    }

    pub fn rotate_from_axis_angle(&mut self, axis: Vec3, angle: f32) {
        self.rotation = Quat::from_axis_angle(axis.normalize(), angle);
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }
}
