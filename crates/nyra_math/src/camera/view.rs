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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_view() {
        let view = look_to_lh(Vec3::ZERO, Vec3::Z, Vec3::Y);

        assert_eq!(view, Mat4::IDENTITY);
    }

    #[test]
    fn camera_translation() {
        let view = look_to_lh(Vec3::new(3.0, -2.0, 5.0), Vec3::Z, Vec3::Y);

        assert_eq!(view.to_cols_array()[3], [-3.0, 2.0, -5.0, 1.0]);
    }

    #[test]
    fn camera_rotation() {
        let view = look_to_lh(Vec3::ZERO, Vec3::X, Vec3::Y);

        let expected = Mat4::from_cols(
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        assert_eq!(view, expected);
    }
}
