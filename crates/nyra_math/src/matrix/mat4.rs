use core::ops::Mul;

use crate::{Quat, Vec3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    cols: [[f32; 4]; 4],
}

impl Mat4 {
    pub const IDENTITY: Self = Self::from_cols(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    );
    pub const ZERO: Self = Self::from_cols_array([[0.0; 4]; 4]);

    pub const fn from_cols(x: [f32; 4], y: [f32; 4], z: [f32; 4], w: [f32; 4]) -> Self {
        let mut cols: [[f32; 4]; 4] = [[0.0; 4]; 4];

        cols[0] = x;
        cols[1] = y;
        cols[2] = z;
        cols[3] = w;

        Self { cols }
    }

    pub const fn from_cols_array(values: [[f32; 4]; 4]) -> Self {
        Self { cols: values }
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let x = rotation * (Vec3::X * scale);
        let y = rotation * (Vec3::Y * scale);
        let z = rotation * (Vec3::Z * scale);

        Self::from_cols_array([
            [x.x, x.y, x.z, 0.0],
            [y.x, y.y, y.z, 0.0],
            [z.x, z.y, z.z, 0.0],
            [translation.x, translation.y, translation.z, 1.0],
        ])
    }

    pub fn to_cols_array(self) -> [[f32; 4]; 4] {
        self.cols
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut cols: [[f32; 4]; 4] = Self::ZERO.cols;

        for col in 0..4 {
            for row in 0..4 {
                for k in 0..4 {
                    cols[col][row] += self.cols[k][row] * rhs.cols[col][k];
                }
            }
        }

        Self::from_cols_array(cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::FRAC_PI_2;

    const EPSILON: f32 = 1e-5;

    fn assert_mat4_close(actual: Mat4, expected: Mat4) {
        let a = actual.to_cols_array();
        let b = expected.to_cols_array();

        for col in 0..4 {
            for row in 0..4 {
                assert!(
                    (a[col][row] - b[col][row]).abs() < EPSILON,
                    "Mismatch at column {col}, row {row}: expected {}, got {}",
                    b[col][row],
                    a[col][row],
                );
            }
        }
    }

    #[test]
    fn constructors() {
        assert_eq!(
            Mat4::IDENTITY.to_cols_array(),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );

        assert_eq!(Mat4::ZERO.to_cols_array(), [[0.0; 4]; 4]);
    }

    #[test]
    fn identity_multiplication() {
        let a = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );

        assert_eq!(a * Mat4::IDENTITY, a);
        assert_eq!(Mat4::IDENTITY * a, a);
        assert_eq!(a * Mat4::ZERO, Mat4::ZERO);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Mat4::from_cols(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        );

        let b = Mat4::from_cols(
            [2.0, 0.0, 1.0, 0.0],
            [0.0, 3.0, 0.0, 0.0],
            [0.0, 0.0, 4.0, 0.0],
            [5.0, -2.0, 1.0, 1.0],
        );

        let expected = Mat4::from_cols(
            [11.0, 14.0, 17.0, 20.0],
            [15.0, 18.0, 21.0, 24.0],
            [36.0, 40.0, 44.0, 48.0],
            [17.0, 22.0, 27.0, 32.0],
        );

        assert_eq!(a * b, expected);
    }

    #[test]
    fn scale_and_translation() {
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(2.0, 3.0, 4.0),
            Quat::IDENTITY,
            Vec3::new(5.0, 6.0, 7.0),
        );

        assert_eq!(
            matrix.to_cols_array(),
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [5.0, 6.0, 7.0, 1.0],
            ]
        );
    }

    #[test]
    fn rotation() {
        let matrix = Mat4::from_scale_rotation_translation(
            Vec3::ONE,
            Quat::from_rotation_y(FRAC_PI_2),
            Vec3::ZERO,
        );

        let expected = Mat4::from_cols(
            [0.0, 0.0, -1.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        assert_mat4_close(matrix, expected);
    }

    #[test]
    fn transformation_order() {
        let translation = Mat4::from_scale_rotation_translation(
            Vec3::ONE,
            Quat::IDENTITY,
            Vec3::new(2.0, 3.0, 4.0),
        );

        let scale = Mat4::from_scale_rotation_translation(
            Vec3::new(2.0, 3.0, 4.0),
            Quat::IDENTITY,
            Vec3::ZERO,
        );

        let translation_then_scale = scale * translation;
        let scale_then_translation = translation * scale;

        assert_eq!(
            translation_then_scale.to_cols_array()[3],
            [4.0, 9.0, 16.0, 1.0]
        );

        assert_eq!(
            scale_then_translation.to_cols_array()[3],
            [2.0, 3.0, 4.0, 1.0]
        );
    }
}
