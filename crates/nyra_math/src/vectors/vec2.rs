use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);

    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);

    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    pub const fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
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
        self.x * other.x + self.y * other.y
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

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        rhs * self
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2> for Vec2 {
    fn sub_assign(&mut self, rhs: Vec2) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Vec2 index out of bounds: {index}"),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Vec2 index out of bounds: {index}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-6;

    #[test]
    fn constructors() {
        assert_eq!(Vec2::ZERO, Vec2::new(0.0, 0.0));
        assert_eq!(Vec2::ONE, Vec2::splat(1.0));
        assert_eq!(Vec2::X, Vec2::new(1.0, 0.0));
        assert_eq!(Vec2::NEG_Y, Vec2::new(0.0, -1.0));
        assert_eq!(Vec2::splat(3.0).to_array(), [3.0, 3.0]);
    }

    #[test]
    fn arithmetic() {
        let a = Vec2::new(2.0, 4.0);
        let b = Vec2::new(1.0, 2.0);

        assert_eq!(a + b, Vec2::new(3.0, 6.0));
        assert_eq!(a - b, Vec2::new(1.0, 2.0));
        assert_eq!(-a, Vec2::new(-2.0, -4.0));

        assert_eq!(a * 2.0, Vec2::new(4.0, 8.0));
        assert_eq!(2.0 * a, Vec2::new(4.0, 8.0));
        assert_eq!(a / 2.0, Vec2::new(1.0, 2.0));

        assert_eq!(a * b, Vec2::new(2.0, 8.0));
        assert_eq!(a / b, Vec2::new(2.0, 2.0));

        let mut c = a;
        c += b;
        c -= b;
        c *= 2.0;
        c /= 2.0;

        assert_eq!(c, a);
    }

    #[test]
    fn geometry() {
        let a = Vec2::ZERO;
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(b.length(), 5.0);
        assert_eq!(b.length_squared(), 25.0);
        assert_eq!(b.dot(Vec2::X), 3.0);
        assert_eq!(Vec2::X.dot(Vec2::Y), 0.0);

        assert_eq!(a.distance(b), 5.0);
        assert_eq!(b.distance(a), 5.0);
        assert_eq!(a.distance_squared(b), 25.0);
    }

    #[test]
    fn normalization() {
        let v = Vec2::new(3.0, 4.0);

        let normalized = v.normalize();

        assert!((normalized.length() - 1.0).abs() < EPSILON);
        assert!((normalized.x - 0.6).abs() < EPSILON);
        assert!((normalized.y - 0.8).abs() < EPSILON);

        assert_eq!(Vec2::ZERO.try_normalize(), None);
        assert_eq!(Vec2::ZERO.normalize_or_zero(), Vec2::ZERO);

        assert!(Vec2::new(f32::NAN, 1.0).try_normalize().is_none());
        assert!(Vec2::new(f32::INFINITY, 1.0).try_normalize().is_none());
    }

    #[test]
    fn indexing() {
        let mut v = Vec2::new(1.0, 2.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);

        v[0] = 5.0;

        assert_eq!(v.x, 5.0);
    }
}
