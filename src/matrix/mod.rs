use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Index, IndexMut, MulAssign, SubAssign};

use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub cols: Vector<Vector<K>>,
}

impl<K> Matrix<K>
{
    pub fn shape(&self) -> (usize, usize) {
        (self.cols.size(), self.cols.elements.first().map_or(0, Vector::size))
    }

    pub fn add(&mut self, other: &Matrix<K>)
        where K: AddAssign + Copy
    {
        assert_eq!(self.shape(), other.shape());
        self.cols.elements.iter_mut().zip(&other.cols.elements).for_each(|(a, b)| a.add(b));
    }

    pub fn sub(&mut self, other: &Matrix<K>)
        where K: SubAssign + Copy
    {
        assert_eq!(self.shape(), other.shape());
        self.cols.elements.iter_mut().zip(&other.cols.elements).for_each(|(a, b)| a.sub(b));
    }

    pub fn scl(&mut self, a: K)
        where K: MulAssign + Copy
    {
        self.cols.elements.iter_mut().for_each(|col| col.scl(a));
    }

    pub fn display(&self)
        where
            K: Debug,
    {
        println!("{}", self);
    }
}


impl<K> From<Vec<Vec<K>>> for Matrix<K>
    where
        K: Copy + Clone,
{
    fn from(array2d: Vec<Vec<K>>) -> Self {
        let vec2d: Vec<Vector<K>> = array2d.into_iter().map(|v| Vector::from(v.as_slice())).collect();
        Matrix { cols: Vector::from(&vec2d) }
    }
}


impl<K, const N1: usize, const N2: usize> From<&[[K; N1]; N2]> for Matrix<K>
    where
        K: Copy + Clone,
{
    fn from(array2d: &[[K; N1]; N2]) -> Self {
        let vec2d: Vec<Vector<K>> = array2d.iter().map(|&col_slice| col_slice.to_vec().into()).collect();
        Matrix { cols: vec2d.into() }
    }
}

impl<K: Debug> Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buffer: String = self.cols.elements.iter().map(|col| format!("{}\n", col)).collect();
        write!(f, "{}", buffer)
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = Vector<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cols[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cols[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_utils() {
        let u = Matrix::from(&[[1., 2.], [3., 4.]]);
        let v = Matrix::from(&[[7., 4.], [-2., 2.]]);

        u.display();
        println!("{}", u);
        println!("{:?}", u);
        println!("{:#?}", u);

        // v.display();
        // println!("{}", v);
        // println!("{:?}", u);
        // println!("{:#?}", v);

        assert_eq!((2, 2), u.shape());
        assert_eq!((2, 2), v.shape());
    }

    #[test]
    fn matrix_add() {
        let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
        let v = Matrix::from(&[[7., 4.], [-2., 2.]]);
        u.add(&v);
        assert_eq!(Vec::from([8.0, 6.0]), u[0].elements);
        assert_eq!(Vec::from([1.0, 6.0]), u[1].elements);

        let mut u = Matrix::from(&[[0, 0], [0, 0]]);
        let v = Matrix::from(&[[0, 0], [0, 0]]);
        u.add(&v);
        assert_eq!(Vec::from([0, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 0]), u[1].elements);

        let mut u = Matrix::from(&[[1, 0], [0, 1]]);
        let v = Matrix::from(&[[0, 0], [0, 0]]);
        u.add(&v);
        assert_eq!(Vec::from([1, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 1]), u[1].elements);

        let mut u = Matrix::from(&[[1, 1], [1, 1]]);
        let v = Matrix::from(&[[1, 1], [1, 1]]);
        u.add(&v);
        assert_eq!(Vec::from([2, 2]), u[0].elements);
        assert_eq!(Vec::from([2, 2]), u[1].elements);

        let mut u = Matrix::from(&[[21, 21], [21, 21]]);
        let v = Matrix::from(&[[21, 21], [21, 21]]);
        u.add(&v);
        assert_eq!(Vec::from([42, 42]), u[0].elements);
        assert_eq!(Vec::from([42, 42]), u[1].elements);
    }

    #[test]
    fn matrix_sub() {
        let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
        let v = Matrix::from(&[[7., 4.], [-2., 2.]]);
        u.sub(&v);
        assert_eq!(Vec::from([-6.0, -2.0]), u[0].elements);
        assert_eq!(Vec::from([5.0, 2.0]), u[1].elements);

        let mut u = Matrix::from(&[[0, 0], [0, 0]]);
        let v = Matrix::from(&[[0, 0], [0, 0]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 0]), u[1].elements);

        let mut u = Matrix::from(&[[1, 0], [0, 1]]);
        let v = Matrix::from(&[[0, 0], [0, 0]]);
        u.sub(&v);
        assert_eq!(Vec::from([1, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 1]), u[1].elements);

        let mut u = Matrix::from(&[[1, 1], [1, 1]]);
        let v = Matrix::from(&[[1, 1], [1, 1]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 0]), u[1].elements);

        let mut u = Matrix::from(&[[21, 21], [21, 21]]);
        let v = Matrix::from(&[[21, 21], [21, 21]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 0]), u[1].elements);
    }

    //
    #[test]
    fn matrix_scale() {
        let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
        u.scl(2.);
        assert_eq!(Vec::from([2.0, 4.0]), u[0].elements);
        assert_eq!(Vec::from([6.0, 8.0]), u[1].elements);

        let mut u = Matrix::from(&[[1, 0], [0, 1]]);
        u.scl(1);
        assert_eq!(Vec::from([1, 0]), u[0].elements);
        assert_eq!(Vec::from([0, 1]), u[1].elements);

        let mut u = Matrix::from(&[[1, 2], [3, 4]]);
        u.scl(2);
        assert_eq!(Vec::from([2, 4]), u[0].elements);
        assert_eq!(Vec::from([6, 8]), u[1].elements);

        let mut u = Matrix::from(&[[21., 21.], [21., 21.]]);
        u.scl(0.5);
        assert_eq!(Vec::from([10.5, 10.5]), u[0].elements);
        assert_eq!(Vec::from([10.5, 10.5]), u[1].elements);
    }

    // #[test]
    // fn matrix_mul_vec() {
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 4.);
    //     assert_eq!(result.cols[1], 2.);
    //
    //     let mut u = Matrix::from(&[[2., 0.], [0., 2.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 8.);
    //     assert_eq!(result.cols[1], 4.);
    //
    //     let mut u = Matrix::from(&[[2., -2.], [-2., 2.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 4.);
    //     assert_eq!(result.cols[1], -4.);
    //
    //     let mut u = Matrix::from(&[[0., 0.], [0., 0.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 0.);
    //     assert_eq!(result.cols[1], 0.);
    //
    //     let mut u = Matrix::from(&[[1., 1.], [1., 1.]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 6.);
    //     assert_eq!(result.cols[1], 6.);
    //
    //     let mut u = Matrix::from(&[[0.5, 0.], [0., 0.5]]);
    //     let v = Vector::from(&[4., 2.]);
    //     let result = u.mul_vec(v);
    //     assert_eq!(result.cols[0], 2.);
    //     assert_eq!(result.cols[1], 1.);
    // }
    //
    // #[test]
    // fn matrix_mul_mat() {
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     let v = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([1., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1.]));
    //
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     let v = Matrix::from(&[[2., 1.], [4., 2.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([2., 1.]));
    //     assert_eq!(result.cols[1], Vec::from([4., 2.]));
    //
    //     let mut u = Matrix::from(&[[3., -5.], [6., 8.]]);
    //     let v = Matrix::from(&[[2., 1.], [4., 2.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([-14., -7.]));
    //     assert_eq!(result.cols[1], Vec::from([44., 22.]));
    //
    //     let mut u = Matrix::from(&[[0., 4., -2.], [-4., -3., 0.]]);
    //     let v = Matrix::from(&[[0., 1.], [1., -1.], [2., 3.]]);
    //     let result = u.mul_mat(v);
    //     assert_eq!(result.cols[0], Vec::from([0., -10.]));
    //     assert_eq!(result.cols[1], Vec::from([-3., -1.]));
    // }
    //
    // #[test]
    // fn matrix_trace_with_zero() {
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     assert_eq!(u.trace(), 2.0);
    //
    //     let mut u = Matrix::from(&[[0., 0.], [0., 0.]]);
    //     assert_eq!(u.trace(), 0.0);
    // }
    //
    // #[test]
    // fn matrix_trace_positive() {
    //     let mut u = Matrix::from(&[[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
    //     assert_eq!(u.trace(), 9.0);
    //
    //     let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
    //     assert_eq!(u.trace(), 5.0);
    //
    //     let mut u = Matrix::from(&[[8., -7.], [4., 2.]]);
    //     assert_eq!(u.trace(), 10.0);
    // }
    //
    // #[test]
    // fn matrix_trace_negative() {
    //     let mut u = Matrix::from(&[[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
    //     assert_eq!(u.trace(), -21.0);
    // }
    //
    // #[test]
    // fn matrix_transpose_zero() {
    //     let mut u = Matrix::from(&[[0., 0.], [0., 0.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 0.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_no_change() {
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_reverse() {
    //     let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 3.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 4.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_bin_no_change() {
    //     let mut u = Matrix::from(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_transpose_positive() {
    //     let mut u = Matrix::from(&[[1., 2., 3.], [4., 5., 6.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 4.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 5.]));
    //     assert_eq!(result.cols[2], Vec::from([3., 6.]));
    //
    //     let mut u = Matrix::from(&[[1., 2.], [3., 4.], [5., 6.]]);
    //     let result = u.transpose();
    //     assert_eq!(result.cols[0], Vec::from([1., 3., 5.]));
    //     assert_eq!(result.cols[1], Vec::from([2., 4., 6.]));
    // }
    //
    // #[test]
    // fn matrix_rref_basics() {
    //     let mut u = Matrix::from(&[[1, -1, 2], [3, 2, 1], [2, -3, -2]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1, 0]));
    //     assert_eq!(result.cols[2], Vec::from([0, 0, 1]));
    //
    //     let mut u = Matrix::from(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1, 0]));
    //     assert_eq!(result.cols[2], Vec::from([0, 0, 1]));
    //
    //     let mut u = Matrix::from(&[[1, 2], [3, 4]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from(&[[1, 2], [2, 4]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 2]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    //
    //     let mut u = Matrix::from(&[[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    //
    //     let mut u = Matrix::from(&[[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0., 0.]));
    //     assert_eq!(result.cols[1], Vec::from([0., 1., 0.]));
    //     assert_eq!(result.cols[2], Vec::from([0., 0., 1.]));
    // }
    //
    // #[test]
    // fn matrix_rref_edge_cases() {
    //     let mut u = Matrix::from(&[[0, 0], [0, 0]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([0, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    //
    //     let mut u = Matrix::from(&[[1, 0], [0, 1]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from(&[[4., 2.], [2., 1.]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1., 0.5]));
    //     assert_eq!(result.cols[1], Vec::from([0., 0.]));
    //
    //     let mut u = Matrix::from(&[[-7, 2], [4, 8]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 0]));
    //     assert_eq!(result.cols[1], Vec::from([0, 1]));
    //
    //     let mut u = Matrix::from(&[[1, 2], [4, 8]]);
    //     let result = u.row_echelon();
    //     assert_eq!(result.cols[0], Vec::from([1, 2]));
    //     assert_eq!(result.cols[1], Vec::from([0, 0]));
    // }
    //
    // #[test]
    // fn matrix_determinant_2() {
    //     let mut u = Matrix::from(&[[1., -1.], [-1., 1.]]);
    //     assert_eq!(u.determinant(), 0.);
    //
    //     let mut u = Matrix::from(&[[0, 0], [0, 0]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 0], [0, 1]]);
    //     assert_eq!(u.determinant(), 1);
    //
    //     let mut u = Matrix::from(&[[2, 0], [0, 2]]);
    //     assert_eq!(u.determinant(), 4);
    //
    //     let mut u = Matrix::from(&[[1, 1], [1, 1]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[0, 1], [1, 0]]);
    //     assert_eq!(u.determinant(), -1);
    //
    //     let mut u = Matrix::from(&[[1, 2], [3, 4]]);
    //     assert_eq!(u.determinant(), -2);
    //
    //     let mut u = Matrix::from(&[[-7, 5], [4, 6]]);
    //     assert_eq!(u.determinant(), -62);
    // }
    //
    // #[test]
    // fn matrix_determinant_3() {
    //     let mut u = Matrix::from(&[[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
    //     assert_eq!(u.determinant(), 8.);
    //
    //     let mut u = Matrix::from(&[[4, 2, 5], [1, 8, 9], [2, 7, 3]]);
    //     assert_eq!(u.determinant(), -171);
    //
    //     let mut u = Matrix::from(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
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
    //     let mut u = Matrix::from(&[[1, 4, 2], [1, 4, 2], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 4, 2], [0, 0, 0], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 4, 2], [3, 9, 5], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 4, 2], [2, 8, 4], [3, 9, 5]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[4, 4, 2], [2, 8, 1], [6, 12, 3]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[4, 4], [4, 4]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 2], [2, 4]]);
    //     assert_eq!(u.determinant(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 2], [0, 0]]);
    //     assert_eq!(u.determinant(), 0);
    // }
    //
    // #[test]
    // fn indentity_matrix_basics() {
    //     let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
    //     let result = u.identity_matrix();
    //     assert_eq!(result.cols[0], vec![1., 0.]);
    //     assert_eq!(result.cols[1], vec![0., 1.]);
    //
    //     let mut u = Matrix::from(&[[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]]);
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
    //     let mut u = Matrix::from(&[[1., 0.], [0., 1.]]);
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
    //     let mut u = Matrix::from(&[[2., 0.], [0., 2.]]);
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
    //     let mut u = Matrix::from(&[[0.5, 0.], [0., 0.5]]);
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
    //     let mut u = Matrix::from(&[[0., 1.], [1., 0.]]);
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
    //     let mut u = Matrix::from(&[[1., 2.], [3., 4.]]);
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
    //     let mut u = Matrix::from(&[[4., 7.], [2., 6.]]);
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
    //     let mut u = Matrix::from(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
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
    //     let mut u = Matrix::from(&[[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);
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
    //     let mut u = Matrix::from(&[[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]);
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
    //     let mut u = Matrix::from(&[[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
    //     assert_eq!(u.rank(), 3);
    //
    //     let mut u = Matrix::from(&[[1, 2, 0, 0], [2, 4, 0, 0], [-1, 2, 1, 1]]);
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
    //     let mut u = Matrix::from(&[[0, 0], [0, 0]]);
    //     assert_eq!(u.rank(), 0);
    //
    //     let mut u = Matrix::from(&[[1, 0], [0, 1]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[[2, 0], [0, 2]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[[1, 1], [1, 1]]);
    //     assert_eq!(u.rank(), 1);
    //
    //     let mut u = Matrix::from(&[[0, 1], [1, 0]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[[1, 2], [3, 4]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[[-7, 5], [4, 6]]);
    //     assert_eq!(u.rank(), 2);
    //
    //     let mut u = Matrix::from(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    //     assert_eq!(u.rank(), 3);
    //
    //     let mut u = Matrix::from(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    //     assert_eq!(u.rank(), 2);
    // }
}
