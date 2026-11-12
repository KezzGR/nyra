use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::{Quat, Vec3, Vec4};

/// A 4×4 `f32` matrix.
///
/// # Representation
///
/// Uses column-major storage and column vectors for transformations.
///
/// # Indexing
///
/// Elements are indexed by `(row, column)`, starting at zero.
///
/// # Examples
///
/// ```
/// use nyra_math::Mat4;
///
/// let mut matrix = Mat4::IDENTITY;
///
/// matrix[(0, 1)] = 5.0;
///
/// assert_eq!(matrix[(0, 1)], 5.0);
/// assert_eq!(matrix[(1, 0)], 0.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    cols: [Vec4; 4],
}

impl Mat4 {
    /// The 4×4 zero matrix.
    pub const ZERO: Self = Self::from_cols_array([[0.0; 4]; 4]);

    /// The 4×4 identity matrix.
    pub const IDENTITY: Self = Self::from_cols(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 1.0),
    );

    /// Creates a matrix from four column vectors.
    pub const fn from_cols(x: Vec4, y: Vec4, z: Vec4, w: Vec4) -> Self {
        Self { cols: [x, y, z, w] }
    }

    /// Creates a matrix from a column-major 2D array.
    pub const fn from_cols_array(cols: [[f32; 4]; 4]) -> Self {
        Self::from_cols(
            Vec4::new(cols[0][0], cols[0][1], cols[0][2], cols[0][3]),
            Vec4::new(cols[1][0], cols[1][1], cols[1][2], cols[1][3]),
            Vec4::new(cols[2][0], cols[2][1], cols[2][2], cols[2][3]),
            Vec4::new(cols[3][0], cols[3][1], cols[3][2], cols[3][3]),
        )
    }

    /// Returns the matrix as a column-major 2D array.
    pub const fn to_cols_array(&self) -> [[f32; 4]; 4] {
        [
            self.cols[0].to_array(),
            self.cols[1].to_array(),
            self.cols[2].to_array(),
            self.cols[3].to_array(),
        ]
    }

    /// Returns the column at the specified index.
    pub const fn col(&self, index: usize) -> Vec4 {
        self.cols[index]
    }

    /// Returns a mutable reference to the column at the specified index.
    pub fn col_mut(&mut self, index: usize) -> &mut Vec4 {
        &mut self.cols[index]
    }

    /// Returns the row at the specified index.
    pub fn row(&self, index: usize) -> Vec4 {
        Vec4::new(
            self.cols[0][index],
            self.cols[1][index],
            self.cols[2][index],
            self.cols[3][index],
        )
    }

    /// Returns the transpose of the matrix.
    pub fn transpose(&self) -> Self {
        Self::from_cols(self.row(0), self.row(1), self.row(2), self.row(3))
    }

    /// Creates a matrix from scale, rotation, and translation.
    ///
    /// Applies scale first, then rotation, then translation (`T * R * S`).
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let x_axis = rotation * (Vec3::X * scale);
        let y_axis = rotation * (Vec3::Y * scale);
        let z_axis = rotation * (Vec3::Z * scale);

        Self::from_cols(
            Vec4::new(x_axis.x, x_axis.y, x_axis.z, 0.0),
            Vec4::new(y_axis.x, y_axis.y, y_axis.z, 0.0),
            Vec4::new(z_axis.x, z_axis.y, z_axis.z, 0.0),
            Vec4::new(translation.x, translation.y, translation.z, 1.0),
        )
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, rhs: Mat4) -> Self::Output {
        Self::from_cols(
            self.cols[0] + rhs.cols[0],
            self.cols[1] + rhs.cols[1],
            self.cols[2] + rhs.cols[2],
            self.cols[3] + rhs.cols[3],
        )
    }
}

impl Sub<Mat4> for Mat4 {
    type Output = Mat4;

    fn sub(self, rhs: Mat4) -> Self::Output {
        Self::from_cols(
            self.cols[0] - rhs.cols[0],
            self.cols[1] - rhs.cols[1],
            self.cols[2] - rhs.cols[2],
            self.cols[3] - rhs.cols[3],
        )
    }
}

impl Neg for Mat4 {
    type Output = Mat4;

    fn neg(self) -> Self::Output {
        Self::from_cols(-self.cols[0], -self.cols[1], -self.cols[2], -self.cols[3])
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4::from_cols(
            self * rhs.cols[0],
            self * rhs.cols[1],
            self * rhs.cols[2],
            self * rhs.cols[3],
        )
    }
}

impl Mul<f32> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut cols = [[0.0; 4]; 4];

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

impl Div<f32> for Mat4 {
    type Output = Mat4;

    fn div(self, rhs: f32) -> Self::Output {
        Self::from_cols(
            self.cols[0] / rhs,
            self.cols[1] / rhs,
            self.cols[2] / rhs,
            self.cols[3] / rhs,
        )
    }
}

impl AddAssign<Mat4> for Mat4 {
    fn add_assign(&mut self, rhs: Mat4) {
        *self = *self + rhs
    }
}

impl SubAssign<Mat4> for Mat4 {
    fn sub_assign(&mut self, rhs: Mat4) {
        *self = *self - rhs
    }
}

impl MulAssign<f32> for Mat4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

impl MulAssign<Mat4> for Mat4 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs
    }
}

impl DivAssign<f32> for Mat4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

impl Index<(usize, usize)> for Mat4 {
    type Output = f32;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (row, column) = idx;

        &self.cols[column][row]
    }
}

impl IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        let (row, column) = idx;

        &mut self.cols[column][row]
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
    fn constants() {
        assert_eq!(Mat4::ZERO.to_cols_array(), [[0.0; 4]; 4]);

        assert_eq!(
            Mat4::IDENTITY.to_cols_array(),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn constructors() {
        let matrix = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        assert_eq!(matrix.to_cols_array(), expected);
        assert_eq!(Mat4::from_cols_array(expected), matrix);
    }

    #[test]
    fn default() {
        assert_eq!(Mat4::default(), Mat4::IDENTITY);
    }

    #[test]
    fn column_access() {
        let mut matrix = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_eq!(matrix.col(1), Vec4::new(5.0, 6.0, 7.0, 8.0));

        *matrix.col_mut(1) = Vec4::new(20.0, 30.0, 40.0, 50.0);

        assert_eq!(matrix.col(1), Vec4::new(20.0, 30.0, 40.0, 50.0));
        assert_eq!(matrix.col(0), Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn row_access() {
        let matrix = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_eq!(matrix.row(0), Vec4::new(1.0, 5.0, 9.0, 13.0));
        assert_eq!(matrix.row(2), Vec4::new(3.0, 7.0, 11.0, 15.0));
    }

    #[test]
    fn matrix_indexing() {
        let mut matrix = Mat4::IDENTITY;

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(1, 2)], 0.0);

        matrix[(1, 2)] = 5.0;

        assert_eq!(matrix[(1, 2)], 5.0);
        assert_eq!(matrix[(2, 1)], 0.0);

        matrix[(3, 1)] = 7.0;

        assert_eq!(matrix[(3, 1)], 7.0);
        assert_eq!(matrix[(1, 3)], 0.0);
    }

    #[test]
    fn matrix_addition() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let b = Mat4::from_cols(
            Vec4::new(2.0, 3.0, 4.0, 5.0),
            Vec4::new(6.0, 7.0, 8.0, 9.0),
            Vec4::new(10.0, 11.0, 12.0, 13.0),
            Vec4::new(14.0, 15.0, 16.0, 17.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(3.0, 5.0, 7.0, 9.0),
            Vec4::new(11.0, 13.0, 15.0, 17.0),
            Vec4::new(19.0, 21.0, 23.0, 25.0),
            Vec4::new(27.0, 29.0, 31.0, 33.0),
        );

        assert_eq!(a + b, expected);
    }

    #[test]
    fn matrix_subtraction() {
        let a = Mat4::from_cols(
            Vec4::new(10.0, 20.0, 30.0, 40.0),
            Vec4::new(50.0, 60.0, 70.0, 80.0),
            Vec4::new(90.0, 100.0, 110.0, 120.0),
            Vec4::new(130.0, 140.0, 150.0, 160.0),
        );

        let b = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(9.0, 18.0, 27.0, 36.0),
            Vec4::new(45.0, 54.0, 63.0, 72.0),
            Vec4::new(81.0, 90.0, 99.0, 108.0),
            Vec4::new(117.0, 126.0, 135.0, 144.0),
        );

        assert_eq!(a - b, expected);
        assert_eq!(a + (-b), expected);
    }

    #[test]
    fn matrix_negation() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, -2.0, 3.0, -4.0),
            Vec4::new(-5.0, 6.0, -7.0, 8.0),
            Vec4::new(9.0, -10.0, 11.0, -12.0),
            Vec4::new(-13.0, 14.0, -15.0, 16.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(-1.0, 2.0, -3.0, 4.0),
            Vec4::new(5.0, -6.0, 7.0, -8.0),
            Vec4::new(-9.0, 10.0, -11.0, 12.0),
            Vec4::new(13.0, -14.0, 15.0, -16.0),
        );

        assert_eq!(-a, expected);
        assert_eq!(-(-a), a);
        assert_eq!(-Mat4::ZERO, Mat4::ZERO);
    }

    #[test]
    fn scalar_multiplication() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(3.0, 6.0, 9.0, 12.0),
            Vec4::new(15.0, 18.0, 21.0, 24.0),
            Vec4::new(27.0, 30.0, 33.0, 36.0),
            Vec4::new(39.0, 42.0, 45.0, 48.0),
        );

        assert_eq!(3.0 * a, expected);
        assert_eq!(a * 3.0, expected);
        assert_eq!(0.0 * a, Mat4::ZERO);
        assert_eq!(1.0 * a, a);
    }

    #[test]
    fn scalar_division() {
        let a = Mat4::from_cols(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_eq!(a / 2.0, expected);
        assert_eq!(a / 1.0, a);
    }

    #[test]
    fn matrix_transpose() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(1.0, 5.0, 9.0, 13.0),
            Vec4::new(2.0, 6.0, 10.0, 14.0),
            Vec4::new(3.0, 7.0, 11.0, 15.0),
            Vec4::new(4.0, 8.0, 12.0, 16.0),
        );

        assert_eq!(a.transpose(), expected);
        assert_eq!(a.transpose().transpose(), a);
        assert_eq!(Mat4::IDENTITY.transpose(), Mat4::IDENTITY);
    }

    #[test]
    fn matrix_multiplication() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let b = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(5.0, -2.0, 1.0, 1.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(11.0, 14.0, 17.0, 20.0),
            Vec4::new(15.0, 18.0, 21.0, 24.0),
            Vec4::new(36.0, 40.0, 44.0, 48.0),
            Vec4::new(17.0, 22.0, 27.0, 32.0),
        );

        assert_eq!(a * b, expected);
    }

    #[test]
    fn identity_multiplication() {
        let a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        assert_eq!(a * Mat4::IDENTITY, a);
        assert_eq!(Mat4::IDENTITY * a, a);
        assert_eq!(a * Mat4::ZERO, Mat4::ZERO);
        assert_eq!(Mat4::ZERO * a, Mat4::ZERO);
    }

    #[test]
    fn matrix_add_assign() {
        let mut a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let b = Mat4::IDENTITY;

        let expected = Mat4::from_cols(
            Vec4::new(2.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 7.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 12.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 17.0),
        );

        a += b;

        assert_eq!(a, expected);
    }

    #[test]
    fn matrix_sub_assign() {
        let mut a = Mat4::from_cols(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );

        let b = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = b;

        a -= b;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_mul_assign() {
        let mut a = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(3.0, 6.0, 9.0, 12.0),
            Vec4::new(15.0, 18.0, 21.0, 24.0),
            Vec4::new(27.0, 30.0, 33.0, 36.0),
            Vec4::new(39.0, 42.0, 45.0, 48.0),
        );

        a *= 3.0;

        assert_eq!(a, expected);
    }

    #[test]
    fn scalar_div_assign() {
        let mut a = Mat4::from_cols(
            Vec4::new(2.0, 4.0, 6.0, 8.0),
            Vec4::new(10.0, 12.0, 14.0, 16.0),
            Vec4::new(18.0, 20.0, 22.0, 24.0),
            Vec4::new(26.0, 28.0, 30.0, 32.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(1.0, 2.0, 3.0, 4.0),
            Vec4::new(5.0, 6.0, 7.0, 8.0),
            Vec4::new(9.0, 10.0, 11.0, 12.0),
            Vec4::new(13.0, 14.0, 15.0, 16.0),
        );

        a /= 2.0;

        assert_eq!(a, expected);
    }

    #[test]
    fn matrix_mul_assign() {
        let mut a = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );

        let b = Mat4::from_cols(
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(1.0, 2.0, 3.0, 1.0),
        );

        let expected = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(2.0, 6.0, 12.0, 1.0),
        );

        a *= b;

        assert_eq!(a, expected);
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
            Vec4::new(0.0, 0.0, -1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
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

        let expected_translation_then_scale = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(4.0, 9.0, 16.0, 1.0),
        );

        let expected_scale_then_translation = Mat4::from_cols(
            Vec4::new(2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 3.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 4.0, 0.0),
            Vec4::new(2.0, 3.0, 4.0, 1.0),
        );

        assert_eq!(translation_then_scale, expected_translation_then_scale);
        assert_eq!(scale_then_translation, expected_scale_then_translation);
    }
}
