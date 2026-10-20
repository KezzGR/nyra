use glam::{
    Mat4,
    camera::lh::{proj, view},
};

use crate::render::Transform;

pub struct Camera {
    pub transform: Transform,
    vertical_fov: f32,
    near: f32,
    far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            vertical_fov: 70.0,
            near: 0.01,
            far: 100.0,
        }
    }
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
        view::look_to_mat4(
            self.transform.position,
            self.transform.forward(),
            self.transform.up(),
        )
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        proj::vulkan::perspective(self.vertical_fov, aspect_ratio, self.near, self.far)
    }
}
