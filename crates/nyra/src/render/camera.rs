use glam::{
    Mat4, Vec3,
    camera::lh::{proj, view},
};

use crate::render::Transform;

pub struct Camera {
    pub transform: Transform,
    vertical_fov: f32,
    near: f32,
    far: f32,
}

impl Camera {
    pub fn new(transform: Transform, vertical_fov: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Transform::new(transform.position, transform.rotation, transform.scale),
            vertical_fov,
            near,
            far,
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        view::look_to_mat4(Vec3::from_array(self.transform.position), Vec3::Z, Vec3::Y)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        proj::vulkan::perspective(self.vertical_fov, aspect_ratio, self.near, self.far)
    }
}
