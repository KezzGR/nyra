use crate::Mat4;

pub fn perspective_lh_zo(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
    let f = 1.0 / (fov_y / 2.0).tan();
    let depth = far / (far - near);

    Mat4::from_cols_array([
        [f / aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, depth, 1.0],
        [0.0, 0.0, -near * depth, 0.0],
    ])
}
