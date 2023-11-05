use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub m: Vector<Vector<K>>,
}

impl<K: std::ops::Deref> std::ops::Deref for Matrix<K> {
    type Target = Vector<Vector<K>>;

    fn deref(&self) -> &Self::Target {
        &self.m
    }
}

impl<K> Add for Matrix<K>
where
    K: AddAssign + Add<Output = K> + Copy, // or whatever bounds you need
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut mat = self.clone();
        for (vec1, vec2) in mat.m.e.iter_mut().zip(rhs.m.e.iter()) {
            vec1.add_mut(vec2);
        }
        mat
    }
}

impl<K> Sub for Matrix<K>
where
    K: SubAssign + Sub<Output = K> + Copy, // or whatever bounds you need
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut mat = self.clone();
        for (vec1, vec2) in mat.m.e.iter_mut().zip(rhs.m.e.iter()) {
            vec1.sub_mut(vec2);
        }
        mat
    }
}

impl<K> Mul<K> for Matrix<K>
where
    K: MulAssign + Copy, // or whatever bounds you need
{
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        let mut mat = self.clone();
        for vec in &mut mat.m.e {
            vec.scl(rhs);
        }
        mat
    }
}

impl<K> Matrix<K> {
    pub fn new(m: Vector<Vector<K>>) -> Self {
        Self { m }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.m.size(), self.m.e.first().map_or(0, Vector::size))
    }

    // MANDATORY -- ex00
    pub fn add_mut(&mut self, other: &Matrix<K>)
    where
        K: AddAssign + Copy,
    {
        assert_eq!(self.shape(), other.shape());
        self.m
            .e
            .iter_mut()
            .zip(&other.m.e)
            .for_each(|(a, b)| a.add_mut(b));
    }
    // MANDATORY -- ex00
    pub fn sub_mut(&mut self, other: &Matrix<K>)
    where
        K: SubAssign + Copy,
    {
        assert_eq!(self.shape(), other.shape());
        self.m
            .e
            .iter_mut()
            .zip(&other.m.e)
            .for_each(|(a, b)| a.sub_mut(b));
    }

    // MANDATORY -- ex00
    pub fn scl(&mut self, a: K)
    where
        K: MulAssign + Copy,
    {
        self.m.e.iter_mut().for_each(|col| col.scl(a));
    }

    pub fn display(&self)
    where
        K: Debug + Display,
    {
        println!("{}", self);
    }

    pub fn m(&self) -> &Vector<Vector<K>> {
        &self.m
    }

    pub fn m_mut(&mut self) -> &mut Vector<Vector<K>> {
        &mut self.m
    }

    pub fn set_m(&mut self, m: Vector<Vector<K>>) {
        self.m = m;
    }
}

impl<K> From<Vec<Vec<K>>> for Matrix<K>
where
    K: Copy + Clone,
{
    fn from(array2d: Vec<Vec<K>>) -> Self {
        let vec2d: Vec<Vector<K>> = array2d
            .into_iter()
            .map(|v| Vector::from(v.as_slice()))
            .collect();
        Matrix {
            m: Vector::from(vec2d),
        }
    }
}

impl<K, const N1: usize, const N2: usize> From<[[K; N1]; N2]> for Matrix<K>
where
    K: Copy + Clone,
{
    fn from(array2d: [[K; N1]; N2]) -> Self {
        let vec2d: Vec<Vector<K>> = array2d
            .iter()
            .map(|&col_slice| col_slice.to_vec().into())
            .collect();
        Matrix { m: vec2d.into() }
    }
}

impl<K> Display for Matrix<K>
where
    K: Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.m.e.iter().enumerate() {
            write!(f, "[")?;
            for (j, element) in v.e.iter().enumerate() {
                if j != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.1}", element)?;
            }
            write!(f, "]")?;
            if i < self.m.e.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = Vector<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m.e[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m.e[index]
    }
}

impl<K> AddAssign<Matrix<K>> for Matrix<K>
where
    K: AddAssign + Copy, // or whatever bounds you need
{
    fn add_assign(&mut self, rhs: Self) {
        for (vec1, vec2) in self.m.e.iter_mut().zip(rhs.m.e.iter()) {
            // *vec1 += *vec2;
            vec1.add_mut(vec2);
        }
    }
}

impl<K> SubAssign<Matrix<K>> for Matrix<K>
where
    K: SubAssign + Copy, // or whatever bounds you need
{
    fn sub_assign(&mut self, rhs: Matrix<K>) {
        for (vec1, vec2) in self.m.e.iter_mut().zip(rhs.m.e.iter()) {
            vec1.sub_mut(vec2);
        }
    }
}

impl<K> MulAssign<K> for Matrix<K>
where
    K: MulAssign + Copy, // or whatever bounds you need
{
    fn mul_assign(&mut self, rhs: K) {
        for vec in &mut self.m.e {
            vec.scl(rhs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_utils() {
        let u = Matrix::from([[1.2, 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);

        u.display();
        println!("{}", u);
        println!("{:?}", u);
        println!("{:#?}", u);

        v.display();
        println!("{}", v);
        println!("{:?}", u);
        println!("{:#?}", v);

        assert_eq!((2, 2), u.shape());
        assert_eq!((2, 2), v.shape());
    }

    #[test]
    fn matrix_add() {
        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u.add_mut(&v);
        assert_eq!(Vec::from([8.0, 6.0]), u[0].e);
        assert_eq!(Vec::from([1.0, 6.0]), u[1].e);

        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u = u.add(v);
        assert_eq!(Vec::from([8.0, 6.0]), u[0].e);
        assert_eq!(Vec::from([1.0, 6.0]), u[1].e);

        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u = u + v;
        assert_eq!(Vec::from([8.0, 6.0]), u[0].e);
        assert_eq!(Vec::from([1.0, 6.0]), u[1].e);

        let mut u = Matrix::from([[1., 2.], [3., 4.]]);
        let v = Matrix::from([[7., 4.], [-2., 2.]]);
        u += v;
        assert_eq!(Vec::from([8.0, 6.0]), u[0].e);
        assert_eq!(Vec::from([1.0, 6.0]), u[1].e);
    }

    #[test]
    fn matrix_sub() {
        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        let v = Matrix::from([[1., 1.], [1., 1.]]);
        u.sub_mut(&v);
        assert_eq!(Vec::from([0., 0.]), u[0].e);
        assert_eq!(Vec::from([0., 0.]), u[1].e);

        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        let v = Matrix::from([[1., 1.], [1., 1.]]);
        u = u.sub(v);
        assert_eq!(Vec::from([0., 0.]), u[0].e);
        assert_eq!(Vec::from([0., 0.]), u[1].e);

        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        let v = Matrix::from([[1., 1.], [1., 1.]]);
        u = u - v;
        assert_eq!(Vec::from([0., 0.]), u[0].e);
        assert_eq!(Vec::from([0., 0.]), u[1].e);

        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        let v = Matrix::from([[1., 1.], [1., 1.]]);
        u -= v;
        assert_eq!(Vec::from([0., 0.]), u[0].e);
        assert_eq!(Vec::from([0., 0.]), u[1].e);
    }

    //
    #[test]
    fn matrix_scale() {
        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        u.scl(42.);
        assert_eq!(Vec::from([42., 42.]), u[0].e);
        assert_eq!(Vec::from([42., 42.]), u[1].e);

        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        u *= 42.;
        assert_eq!(Vec::from([42., 42.]), u[0].e);
        assert_eq!(Vec::from([42., 42.]), u[1].e);

        let mut u = Matrix::from([[1., 1.], [1., 1.]]);
        u = u * 42.;
        assert_eq!(Vec::from([42., 42.]), u[0].e);
        assert_eq!(Vec::from([42., 42.]), u[1].e);
    }

    // #[test]
    // fn matrix_mul_vec() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 4.);
    //     assert_eq!(result.cols[1], 2.);
    //
    //     let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 8.);
    //     assert_eq!(result.cols[1], 4.);
    //
    //     let mut u = Matrix::from([[2., -2.], [-2., 2.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 4.);
    //     assert_eq!(result.cols[1], -4.);
    //
    //     let mut u = Matrix::from([[0., 0.], [0., 0.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 0.);
    //     assert_eq!(result.cols[1], 0.);
    //
    //     let mut u = Matrix::from([[1., 1.], [1., 1.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 6.);
    //     assert_eq!(result.cols[1], 6.);
    //
    //     let mut u = Matrix::from([[0.5, 0.], [0., 0.5]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 2.);
    //     assert_eq!(result.cols[1], 1.);
    // }
    //
    // #[test]
    // fn matrix_mul_mat() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let v = Matrix::from([[1., 0.], [0., 1.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([1., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1.]));
    //
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let v = Matrix::from([[2., 1.], [4., 2.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([2., 1.]));
    //     assert_eq!(result.cols[1], Vec::from([4., 2.]));
    //
    //     let mut u = Matrix::from([[3., -5.], [6., 8.]]);
    //     let v = Matrix::from([[2., 1.], [4., 2.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([-14., -7.]));
    //     assert_eq!(result.cols[1], Vec::from([44., 22.]));
    //
    //     let mut u = Matrix::from([[0., 4., -2.], [-4., -3., 0.]]);
    //     let v = Matrix::from([[0., 1.], [1., -1.], [2., 3.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([0., -10.]));
    //     assert_eq!(result.cols[1], Vec::from([-3., -1.]));
    // }
    //
    // #[test]
    // fn matrix_trace_with_zero() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     assert_eq!(u.trace(), 2.0);
    //
    //     let mut u = Matrix::from([[0., 0.], [0., 0.]]);
    //     assert_eq!(u.trace(), 0.0);
    // }
    //
    // #[test]
    // fn matrix_trace_positive() {
    //     let mut u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    //     assert_eq!(u.trace(), 9.0);
    //
    //     let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    //     assert_eq!(u.trace(), 5.0);
    //
    //     let mut u = Matrix::from([[8., -7.], [4., 2.]]);
    //     assert_eq!(u.trace(), 10.0);
    // }
    //
    // #[test]
    // fn matrix_trace_negative() {
    //     let mut u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    //     assert_eq!(u.trace(), -21.0);
    // }
    //
    // #[test]
    // fn matrix_transpose_zero() {
    //     let mut u = Matrix::from([[0., 0.], [0., 0.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 0.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_no_change() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_reverse() {
    //     let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 3.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 4.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_bin_no_change() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_positive() {
    //     let mut u = Matrix::from([[1., 2., 3.], [4., 5., 6.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 4.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 5.]));
    //     assert_eq!(result.cols[2], Vec::from([3., 6.]));
    //
    //     let mut u = Matrix::from([[1., 2.], [3., 4.], [5., 6.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 3., 5.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 4., 6.]));
    // }
    //
    // #[test]
    // fn matrix_rref_basics() {
    //     let mut u = Matrix::from([[1, -1, 2], [3, 2, 1], [2, -3, -2]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1, 0]));
    //     assert_eq!(result.cols[2], Vec::from([0, 0, 1]));
    //
    //     let mut u = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1, 0]));
    //     assert_eq!(result.cols[2], Vec::from([0, 0, 1]));
    //
    //     let mut u = Matrix::from([[1, 2], [3, 4]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from([[1, 2], [2, 4]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 2]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    //
    //     let mut u = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    //
    //     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_rref_edge_cases() {
    //     let mut u = Matrix::from([[0, 0], [0, 0]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    //
    //     let mut u = Matrix::from([[1, 0], [0, 1]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from([[4., 2.], [2., 1.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0.5]));
    //     assert_eq!(result.cols[1], Vec::from([0., 0.]));
    //
    //     let mut u = Matrix::from([[-7, 2], [4, 8]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from([[1, 2], [4, 8]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 2]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    // }
    //
    // #[test]
    // fn matrix_determinant_2() {
    //     let mut u = Matrix::from([[1., -1.], [-1., 1.]]);
    //     assert_eq!(u.determinant(), 0.);
    //
    //     let mut u = Matrix::from([[0, 0], [0, 0]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 0], [0, 1]]);
    //     assert_eq!(u.determinant(), 1);
    //
    //     let mut u = Matrix::from([[2, 0], [0, 2]]);
    //     assert_eq!(u.determinant(), 4);
    //
    //     let mut u = Matrix::from([[1, 1], [1, 1]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[0, 1], [1, 0]]);
    //     assert_eq!(u.determinant(), -1);
    //
    //     let mut u = Matrix::from([[1, 2], [3, 4]]);
    //     assert_eq!(u.determinant(), -2);
    //
    //     let mut u = Matrix::from([[-7, 5], [4, 6]]);
    //     assert_eq!(u.determinant(), -62);
    // }
    //
    // #[test]
    // fn matrix_determinant_3() {
    //     let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    //     assert_eq!(u.determinant(), 8.);
    //
    //     let mut u = Matrix::from([[4, 2, 5], [1, 8, 9], [2, 7, 3]]);
    //     assert_eq!(u.determinant(), -171);
    //
    //     let mut u = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    //     assert_eq!(u.determinant(), 1);
    // }
    //
    // #[test]
    // fn matrix_determinant_4() {
    //     let mut u = Matrix::from(&[
    //         &[8., 5., -2., 4.],
    //         &[4., 2.5, 20., 4.],
    //         &[8., 5., 1., 4.],
    //         &[28., -4., 17., 1.],
    //     ]);
    //
    //     assert_eq!(u.determinant(), 1032.);
    //
    //     let mut u = Matrix::from(&[
    //         &[1, 1, 1, -1],
    //         &[1, 1, -1, 1],
    //         &[1, -1, 1, 1],
    //         &[-1, 1, 1, 1],
    //     ]);
    //
    //     assert_eq!(u.determinant(), -16);
    // }
    //
    // #[test]
    // fn matrix_determinant_0() {
    //     let mut u = Matrix::from([[1, 4, 2], [1, 4, 2], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 4, 2], [0, 0, 0], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 4, 2], [3, 9, 5], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 4, 2], [2, 8, 4], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[4, 4, 2], [2, 8, 1], [6, 12, 3]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[4, 4], [4, 4]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 2], [2, 4]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from([[1, 2], [0, 0]]);
    //     assert_eq!(u.determinant(), 0);
    // }
    //
    // #[test]
    // fn indentity_matrix_basics() {
    //     let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    //     let result = u.identity_matrix();
    //     assert_eq!(result.cols[0], vec![1., 0.]);
    //     assert_eq!(result.cols[1], vec![0., 1.]);
    //
    //     let mut u = Matrix::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    //     let result = u.identity_matrix();
    //     assert_eq!(result.cols[0], vec![1., 0., 0.]);
    //     assert_eq!(result.cols[1], vec![0., 1., 0.]);
    //     assert_eq!(result.cols[2], vec![0., 0., 1.]);
    //
    //     let mut u = Matrix::from(&[
    //         &[1., 2., 3., 4.],
    //         &[5., 6., 7., 8.],
    //         &[8., 9., 10., 11.],
    //         &[12., 13., 14., 15.],
    //     ]);
    //     let result = u.identity_matrix();
    //     assert_eq!(result.cols[0], vec![1., 0., 0., 0.]);
    //     assert_eq!(result.cols[1], vec![0., 1., 0., 0.]);
    //     assert_eq!(result.cols[2], vec![0., 0., 1., 0.]);
    //     assert_eq!(result.cols[3], vec![0., 0., 0., 1.]);
    // }
    //
    // #[test]
    // fn inverse_matrix_2x2() {
    //     let mut u = Matrix::from([[1., 0.], [0., 1.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![1., 0.]);
    //             assert_eq!(r.cols[1], vec![0., 1.]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[2., 0.], [0., 2.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![0.5, 0.]);
    //             assert_eq!(r.cols[1], vec![0., 0.5]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[0.5, 0.], [0., 0.5]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![2., 0.]);
    //             assert_eq!(r.cols[1], vec![0., 2.]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[0., 1.], [1., 0.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![0., 1.]);
    //             assert_eq!(r.cols[1], vec![1., 0.]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[1., 2.], [3., 4.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![-2., 1.]);
    //             assert_eq!(r.cols[1], vec![1.5, -0.5]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[4., 7.], [2., 6.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![0.6, -0.7]);
    //             assert_eq!(r.cols[1], vec![-0.2, 0.4]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    // }
    //
    // #[test]
    // fn inverse_matrix_3x3() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![1., 0., 0.]);
    //             assert_eq!(r.cols[1], vec![0., 1., 0.]);
    //             assert_eq!(r.cols[2], vec![0., 0., 1.]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![0.5, 0., 0.]);
    //             assert_eq!(r.cols[1], vec![0., 0.5, 0.]);
    //             assert_eq!(r.cols[2], vec![0., 0., 0.5]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    //
    //     let mut u = Matrix::from([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    //     let result = u.inverse();
    //     match result {
    //         Ok(r) => {
    //             assert_eq!(r.cols[0], vec![0.649425287, 0.097701149, -0.655172414]);
    //             assert_eq!(
    //                 r.cols[1],
    //                 vec![-0.781609195, -0.126436782, 0.965517241]
    //             );
    //             assert_eq!(r.cols[2], vec![0.143678161, 0.07471265, -0.206896552]);
    //         }
    //         Err(_) => {
    //             assert_eq!(0, 1);
    //         }
    //     }
    // }
    //
    // #[test]
    // fn matrix_rank() {
    //     let mut u = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     assert_eq!(u.rank(), 3);
    //
    //     let mut u = Matrix::from([[1, 2, 0, 0], [2, 4, 0, 0], [-1, 2, 1, 1]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[
    //         &[8., 5., -2.],
    //         &[4., 7., 20.],
    //         &[7., 6., 1.],
    //         &[21., 18., 7.],
    //     ]);
    //     assert_eq!(u.rank(), 3);
    //
    //     let mut u = Matrix::from([[0, 0], [0, 0]]);
    //     assert_eq!(u.rank(), 0);
    //
    //     let mut u = Matrix::from([[1, 0], [0, 1]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from([[2, 0], [0, 2]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from([[1, 1], [1, 1]]);
    //     assert_eq!(u.rank(), 1);
    //
    //     let mut u = Matrix::from([[0, 1], [1, 0]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from([[1, 2], [3, 4]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from([[-7, 5], [4, 6]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    //     assert_eq!(u.rank(), 3);
    //
    //     let mut u = Matrix::from([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    //     assert_eq!(u.rank(), 2);
    // }
}
