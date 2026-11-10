use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub const fn to_array(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
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
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
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

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, rhs: Vec4) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Vec4) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Neg for Vec4 {
    type Output = Vec4;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z,
            self.w * rhs.w,
        )
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Div<Vec4> for Vec4 {
    type Output = Vec4;

    fn div(self, rhs: Vec4) -> Self::Output {
        Self::new(
            self.x / rhs.x,
            self.y / rhs.y,
            self.z / rhs.z,
            self.w / rhs.w,
        )
    }
}

impl AddAssign<Vec4> for Vec4 {
    fn add_assign(&mut self, rhs: Vec4) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign<Vec4> for Vec4 {
    fn sub_assign(&mut self, rhs: Vec4) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl MulAssign<Vec4> for Vec4 {
    fn mul_assign(&mut self, rhs: Vec4) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl DivAssign<Vec4> for Vec4 {
    fn div_assign(&mut self, rhs: Vec4) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Vec4 index out of bounds: {index}"),
        }
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Vec4 index out of bounds: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    fn assert_vec4_approx_eq(a: Vec4, b: Vec4) {
        for i in 0..4 {
            assert!(
                (a[i] - b[i]).abs() < EPSILON,
                "component {i}: {} != {}",
                a[i],
                b[i]
            );
        }
    }

    #[test]
    fn arithmetic() {
        let a = Vec4::new(2.0, 4.0, 6.0, 8.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(a + b, Vec4::new(3.0, 6.0, 9.0, 12.0));
        assert_eq!(a - b, Vec4::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(-a, Vec4::new(-2.0, -4.0, -6.0, -8.0));

        assert_eq!(a * 2.0, Vec4::new(4.0, 8.0, 12.0, 16.0));
        assert_eq!(2.0 * a, a * 2.0);
        assert_eq!(a / 2.0, Vec4::new(1.0, 2.0, 3.0, 4.0));

        assert_eq!(a * b, Vec4::new(2.0, 8.0, 18.0, 32.0));
        assert_eq!(a / b, Vec4::splat(2.0));
    }

    #[test]
    fn assignment_operators() {
        let a = Vec4::new(2.0, 4.0, 6.0, 8.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 4.0);

        let mut v = a;
        v += b;
        assert_eq!(v, a + b);

        v = a;
        v -= b;
        assert_eq!(v, a - b);

        v = a;
        v *= 2.0;
        assert_eq!(v, a * 2.0);

        v = a;
        v *= b;
        assert_eq!(v, a * b);

        v = a;
        v /= 2.0;
        assert_eq!(v, a / 2.0);

        v = a;
        v /= b;
        assert_eq!(v, a / b);
    }

    #[test]
    fn dot_product() {
        let a = Vec4::new(1.0, -2.0, 3.0, -4.0);
        let b = Vec4::new(2.0, 3.0, -1.0, -2.0);

        assert_eq!(a.dot(b), 1.0);
        assert_eq!(Vec4::X.dot(Vec4::Y), 0.0);
        assert_eq!(Vec4::W.dot(Vec4::W), 1.0);
    }

    #[test]
    fn length_and_distance() {
        let a = Vec4::new(4.0, 6.0, 3.0, 1.0);
        let b = Vec4::new(1.0, 2.0, 3.0, 1.0);

        assert_eq!(a.length_squared(), 62.0);
        assert!((a.length() - 62.0_f32.sqrt()).abs() < EPSILON);

        assert_eq!(a.distance_squared(b), 25.0);
        assert_eq!(a.distance(b), 5.0);
    }

    #[test]
    fn normalization() {
        let v = Vec4::new(3.0, 4.0, 0.0, 0.0);

        let expected = Vec4::new(0.6, 0.8, 0.0, 0.0);

        assert_vec4_approx_eq(v.normalize(), expected);
        assert_vec4_approx_eq(v.try_normalize().unwrap(), expected);
        assert_vec4_approx_eq(v.normalize_or_zero(), expected);
    }

    #[test]
    fn normalization_edge_cases() {
        assert_eq!(Vec4::ZERO.try_normalize(), None);
        assert_eq!(Vec4::ZERO.normalize_or_zero(), Vec4::ZERO);

        let infinity = Vec4::new(f32::INFINITY, 1.0, 0.0, 0.0);
        assert_eq!(infinity.try_normalize(), None);

        let nan = Vec4::new(f32::NAN, 1.0, 0.0, 0.0);
        assert_eq!(nan.try_normalize(), None);
    }

    #[test]
    fn indexing() {
        let mut v = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);

        v[2] = 10.0;
        assert_eq!(v.z, 10.0);
    }

    #[test]
    #[should_panic(expected = "Vec4 index out of bounds")]
    fn indexing_out_of_bounds() {
        let v = Vec4::ZERO;
        let _ = v[4];
    }
}
