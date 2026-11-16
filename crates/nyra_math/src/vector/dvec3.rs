use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);

    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub const fn splat(value: f64) -> Self {
        Self::new(value, value, value)
    }

    pub const fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Self) -> f64 {
        (self - other).length_squared()
    }

    pub fn dot(self, other: Self) -> f64 {
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

impl Add<DVec3> for DVec3 {
    type Output = DVec3;

    fn add(self, rhs: DVec3) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<DVec3> for DVec3 {
    type Output = DVec3;

    fn sub(self, rhs: DVec3) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for DVec3 {
    type Output = DVec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for DVec3 {
    type Output = DVec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<DVec3> for f64 {
    type Output = DVec3;

    fn mul(self, rhs: DVec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for DVec3 {
    type Output = DVec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Mul<DVec3> for DVec3 {
    type Output = DVec3;

    fn mul(self, rhs: DVec3) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Div<DVec3> for DVec3 {
    type Output = DVec3;

    fn div(self, rhs: DVec3) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl AddAssign<DVec3> for DVec3 {
    fn add_assign(&mut self, rhs: DVec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign<DVec3> for DVec3 {
    fn sub_assign(&mut self, rhs: DVec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<f64> for DVec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for DVec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for DVec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("DVec3 index out of bounds: {index}"),
        }
    }
}

impl IndexMut<usize> for DVec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("DVec3 index out of bounds: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-12;

    #[test]
    fn constructors() {
        assert_eq!(DVec3::ZERO, DVec3::new(0.0, 0.0, 0.0));
        assert_eq!(DVec3::ONE, DVec3::splat(1.0));
        assert_eq!(DVec3::X, DVec3::new(1.0, 0.0, 0.0));
        assert_eq!(DVec3::NEG_Z, DVec3::new(0.0, 0.0, -1.0));
        assert_eq!(DVec3::splat(2.0).to_array(), [2.0, 2.0, 2.0]);
    }

    #[test]
    fn arithmetic() {
        let a = DVec3::new(2.0, 4.0, 6.0);
        let b = DVec3::new(1.0, 2.0, 3.0);

        assert_eq!(a + b, DVec3::new(3.0, 6.0, 9.0));
        assert_eq!(a - b, b);
        assert_eq!(-a, DVec3::new(-2.0, -4.0, -6.0));

        assert_eq!(a * 2.0, DVec3::new(4.0, 8.0, 12.0));
        assert_eq!(2.0 * a, DVec3::new(4.0, 8.0, 12.0));
        assert_eq!(a / 2.0, b);

        assert_eq!(a * b, DVec3::new(2.0, 8.0, 18.0));
        assert_eq!(a / b, DVec3::splat(2.0));

        let mut c = a;
        c += b;
        c -= b;
        c *= 2.0;
        c /= 2.0;

        assert_eq!(c, a);
    }

    #[test]
    fn cross_product() {
        assert_eq!(DVec3::X.cross(DVec3::Y), DVec3::Z);
        assert_eq!(DVec3::Y.cross(DVec3::X), DVec3::NEG_Z);
        assert_eq!(DVec3::X.cross(DVec3::X), DVec3::ZERO);

        let a = DVec3::new(1.0, 2.0, 3.0);
        let b = DVec3::new(4.0, 5.0, 6.0);

        let cross = a.cross(b);

        assert_eq!(cross, DVec3::new(-3.0, 6.0, -3.0));

        assert_eq!(cross.dot(a), 0.0);
        assert_eq!(cross.dot(b), 0.0);

        assert_eq!(a.cross(b), -b.cross(a));
    }

    #[test]
    fn geometry() {
        let a = DVec3::ZERO;
        let b = DVec3::new(3.0, 4.0, 0.0);

        assert_eq!(b.length(), 5.0);
        assert_eq!(b.length_squared(), 25.0);
        assert_eq!(b.dot(DVec3::X), 3.0);

        assert_eq!(a.distance(b), 5.0);
        assert_eq!(b.distance(a), 5.0);
        assert_eq!(a.distance_squared(b), 25.0);
    }

    #[test]
    fn normalization() {
        let v = DVec3::new(3.0, 4.0, 0.0);

        let normalized = v.normalize();

        assert!((normalized.length() - 1.0).abs() < EPSILON);
        assert!((normalized.x - 0.6).abs() < EPSILON);
        assert!((normalized.y - 0.8).abs() < EPSILON);

        assert_eq!(DVec3::ZERO.try_normalize(), None);
        assert_eq!(DVec3::ZERO.normalize_or_zero(), DVec3::ZERO);

        assert!(DVec3::new(f64::NAN, 0.0, 0.0).try_normalize().is_none());
        assert!(
            DVec3::new(f64::INFINITY, 0.0, 0.0)
                .try_normalize()
                .is_none()
        );
    }

    #[test]
    fn indexing() {
        let mut v = DVec3::new(1.0, 2.0, 3.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        v[2] = 10.0;

        assert_eq!(v.z, 10.0);
    }
}
