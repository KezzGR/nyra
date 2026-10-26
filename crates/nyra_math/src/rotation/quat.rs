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

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::{FRAC_PI_2, PI};

    const EPSILON: f32 = 1e-5;

    fn assert_vec3_close(actual: Vec3, expected: Vec3) {
        assert!(
            (actual - expected).length() < EPSILON,
            "expected {expected:?}, got {actual:?}"
        );
    }

    #[test]
    fn identity() {
        let q = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let v = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(q * Quat::IDENTITY, q);
        assert_eq!(Quat::IDENTITY * q, q);
        assert_eq!(Quat::IDENTITY * v, v);
    }

    #[test]
    fn quaternion_multiplication() {
        let a = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
        let b = Quat::from_xyzw(5.0, 6.0, 7.0, 8.0);

        assert_eq!(a * b, Quat::from_xyzw(24.0, 48.0, 48.0, -6.0));
    }

    #[test]
    fn axis_angle() {
        let q = Quat::from_axis_angle(Vec3::Y, PI);

        assert!(q.x.abs() < EPSILON);
        assert!((q.y - 1.0).abs() < EPSILON);
        assert!(q.z.abs() < EPSILON);
        assert!(q.w.abs() < EPSILON);

        assert_vec3_close(Quat::from_axis_angle(Vec3::X, 0.0) * Vec3::Y, Vec3::Y);
    }

    #[test]
    fn rotations() {
        assert_vec3_close(Quat::from_rotation_x(FRAC_PI_2) * Vec3::Y, Vec3::Z);

        assert_vec3_close(Quat::from_rotation_y(FRAC_PI_2) * Vec3::Z, Vec3::X);

        assert_vec3_close(Quat::from_rotation_z(FRAC_PI_2) * Vec3::X, Vec3::Y);
    }

    #[test]
    fn rotation_composition() {
        let a = Quat::from_rotation_x(0.5);
        let b = Quat::from_rotation_y(0.8);
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_vec3_close((a * b) * v, a * (b * v));
    }

    #[test]
    fn multiplication_is_not_commutative() {
        let x = Quat::from_rotation_x(FRAC_PI_2);
        let y = Quat::from_rotation_y(FRAC_PI_2);

        assert_vec3_close((x * y) * Vec3::Z, Vec3::X);
        assert_vec3_close((y * x) * Vec3::Z, Vec3::NEG_Y);
    }

    #[test]
    fn rotation_preserves_length() {
        let axis = Vec3::new(1.0, 2.0, 3.0).normalize();
        let rotation = Quat::from_axis_angle(axis, 0.7);
        let inverse_rotation = Quat::from_axis_angle(axis, -0.7);

        let v = Vec3::new(2.0, -3.0, 4.0);
        let rotated = rotation * v;

        assert!((rotated.length() - v.length()).abs() < EPSILON);

        assert_vec3_close(inverse_rotation * rotated, v);
    }
}
