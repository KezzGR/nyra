use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value)
    }

    pub const fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn normalize(self) -> Self {
        self / self.length()
    }

    pub fn try_normalize(self) -> Option<Self> {
        let length = self.length();

        if length > 0.0 && length.is_finite() {
            Some(self / length)
        } else {
            None
        }
    }

    pub fn normalize_or_zero(self) -> Self {
        self.try_normalize().unwrap_or(Self::ZERO)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of bounds: {index}"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of bounds: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn constructors() {
        assert_eq!(Vec3::ZERO, Vec3::new(0.0, 0.0, 0.0));
        assert_eq!(Vec3::ONE, Vec3::splat(1.0));
        assert_eq!(Vec3::X, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(Vec3::NEG_Z, Vec3::new(0.0, 0.0, -1.0));
        assert_eq!(Vec3::splat(2.0).to_array(), [2.0, 2.0, 2.0]);
    }

    #[test]
    fn arithmetic() {
        let a = Vec3::new(2.0, 4.0, 6.0);
        let b = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(a + b, Vec3::new(3.0, 6.0, 9.0));
        assert_eq!(a - b, b);
        assert_eq!(-a, Vec3::new(-2.0, -4.0, -6.0));

        assert_eq!(a * 2.0, Vec3::new(4.0, 8.0, 12.0));
        assert_eq!(2.0 * a, Vec3::new(4.0, 8.0, 12.0));
        assert_eq!(a / 2.0, b);

        assert_eq!(a * b, Vec3::new(2.0, 8.0, 18.0));
        assert_eq!(a / b, Vec3::splat(2.0));

        let mut c = a;
        c += b;
        c -= b;
        c *= 2.0;
        c /= 2.0;

        assert_eq!(c, a);
    }

    #[test]
    fn cross_product() {
        assert_eq!(Vec3::X.cross(Vec3::Y), Vec3::Z);
        assert_eq!(Vec3::Y.cross(Vec3::X), Vec3::NEG_Z);
        assert_eq!(Vec3::X.cross(Vec3::X), Vec3::ZERO);

        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let cross = a.cross(b);

        assert_eq!(cross, Vec3::new(-3.0, 6.0, -3.0));

        assert_eq!(cross.dot(a), 0.0);
        assert_eq!(cross.dot(b), 0.0);

        assert_eq!(a.cross(b), -b.cross(a));
    }

    #[test]
    fn geometry() {
        let a = Vec3::ZERO;
        let b = Vec3::new(3.0, 4.0, 0.0);

        assert_eq!(b.length(), 5.0);
        assert_eq!(b.length_squared(), 25.0);
        assert_eq!(b.dot(Vec3::X), 3.0);

        assert_eq!(a.distance(b), 5.0);
        assert_eq!(b.distance(a), 5.0);
        assert_eq!(a.distance_squared(b), 25.0);
    }

    #[test]
    fn normalization() {
        let v = Vec3::new(3.0, 4.0, 0.0);

        let normalized = v.normalize();

        assert!((normalized.length() - 1.0).abs() < EPSILON);
        assert!((normalized.x - 0.6).abs() < EPSILON);
        assert!((normalized.y - 0.8).abs() < EPSILON);

        assert_eq!(Vec3::ZERO.try_normalize(), None);
        assert_eq!(Vec3::ZERO.normalize_or_zero(), Vec3::ZERO);

        assert!(Vec3::new(f32::NAN, 0.0, 0.0).try_normalize().is_none());
        assert!(Vec3::new(f32::INFINITY, 0.0, 0.0).try_normalize().is_none());
    }

    #[test]
    fn indexing() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        v[2] = 10.0;

        assert_eq!(v.z, 10.0);
    }
}