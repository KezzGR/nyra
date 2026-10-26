use crate::{Mat4, Vec3};

pub fn look_to_lh(camera_position: Vec3, camera_direction: Vec3, up: Vec3) -> Mat4 {
    let forward = camera_direction.normalize();
    let right = up.cross(forward).normalize();
    let up = forward.cross(right);

    Mat4::from_cols_array([
        [right.x, up.x, forward.x, 0.0],
        [right.y, up.y, forward.y, 0.0],
        [right.z, up.z, forward.z, 0.0],
        [
            -right.dot(camera_position),
            -up.dot(camera_position),
            -forward.dot(camera_position),
            1.0,
        ],
    ])
}
