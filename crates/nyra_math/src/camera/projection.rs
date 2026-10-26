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

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::FRAC_PI_2;

    const EPSILON: f32 = 1e-5;

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn perspective_matrix() {
        let projection = perspective_lh_zo(FRAC_PI_2, 2.0, 1.0, 11.0);

        let cols = projection.to_cols_array();

        assert_close(cols[0][0], 0.5);
        assert_close(cols[1][1], 1.0);
        assert_close(cols[2][2], 1.1);
        assert_close(cols[2][3], 1.0);
        assert_close(cols[3][2], -1.1);
        assert_close(cols[3][3], 0.0);
    }

    #[test]
    fn depth_range() {
        let near = 1.0;
        let far = 10.0;

        let projection = perspective_lh_zo(FRAC_PI_2, 1.0, near, far);

        let cols = projection.to_cols_array();

        let project_depth = |z: f32| {
            let clip_z = cols[2][2] * z + cols[3][2];
            let clip_w = cols[2][3] * z + cols[3][3];

            clip_z / clip_w
        };

        assert_close(project_depth(near), 0.0);
        assert_close(project_depth(far), 1.0);
    }

    #[test]
    fn aspect_ratio() {
        let projection = perspective_lh_zo(FRAC_PI_2, 2.0, 0.1, 100.0);

        let cols = projection.to_cols_array();

        assert_close(cols[1][1] / cols[0][0], 2.0);
    }
}
