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

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, transtation: Vec3) -> Self {
        let x = rotation * (Vec3::X * scale);
        let y = rotation * (Vec3::Y * scale);
        let z = rotation * (Vec3::Z * scale);

        Self::from_cols_array([
            [x.x, x.y, x.z, 0.0],
            [y.x, y.y, y.z, 0.0],
            [z.x, z.y, z.z, 0.0],
            [transtation.x, transtation.y, transtation.z, 1.0],
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
