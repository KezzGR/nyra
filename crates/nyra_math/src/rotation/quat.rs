use core::ops::Mul;

use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quat {
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        Self::from_axis_angle(Vec3::X, angle)
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        Self::from_axis_angle(Vec3::Y, angle)
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        Self::from_axis_angle(Vec3::Z, angle)
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        let half_angle = angle / 2.0;
        let half_angle_sin = half_angle.sin();

        Self::from_xyzw(
            axis.x * half_angle_sin,
            axis.y * half_angle_sin,
            axis.z * half_angle_sin,
            half_angle.cos(),
        )
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let u = Vec3::new(self.x, self.y, self.z);
        let v = Vec3::new(rhs.x, rhs.y, rhs.z);

        let scalar = self.w * rhs.w - u.dot(v);
        let vector = self.w * v + rhs.w * u + u.cross(v);

        Self::from_xyzw(vector.x, vector.y, vector.z, scalar)
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let u = Vec3::new(self.x, self.y, self.z);

        let t = 2.0 * u.cross(rhs);

        rhs + self.w * t + u.cross(t)
    }
}
