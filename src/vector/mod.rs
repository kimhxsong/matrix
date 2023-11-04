use core::fmt;
// use core::ops;
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub e: Vec<K>,
}

impl<K: std::ops::Deref> std::ops::Deref for Vector<K> {
    type Target = Vec<K>;

    fn deref(&self) -> &Self::Target {
        &self.e
    }
}

impl<K> AddAssign<Vector<K>> for Vector<K>
where
    K: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Vector<K>) {
        self.e
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<K> SubAssign<Vector<K>> for Vector<K>
where
    K: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<K>) {
        self.e
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<K> MulAssign<K> for Vector<K>
where
    K: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: K) {
        let scalar = K::from(rhs);
        self.e.iter_mut().for_each(|e| *e *= scalar);
    }
}

impl<K> AddAssign<&Vector<K>> for Vector<K>
where
    K: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &Vector<K>) {
        self.e
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<K> SubAssign<&Vector<K>> for Vector<K>
where
    K: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: &Vector<K>) {
        self.e
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<K> MulAssign<&K> for Vector<K>
where
    K: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: &K) {
        self.e.iter_mut().for_each(|e| *e *= *rhs);
    }
}

impl<K> Add<Vector<K>> for Vector<K>
where
    K: AddAssign + Add<Output = K> + Copy, // or whatever bounds you need
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for (p1, p2) in vec.e.iter_mut().zip(rhs.e.iter()) {
            *p1 += *p2;
        }
        vec
    }
}

impl<K> Sub<Vector<K>> for Vector<K>
where
    K: SubAssign + Copy, // or whatever bounds you need
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut vec = self.clone();
        for (p1, p2) in vec.e.iter_mut().zip(rhs.e.iter()) {
            *p1 -= *p2;
        }
        vec
    }
}

impl<K> Mul<K> for Vector<K>
where
    K: MulAssign + Copy, // or whatever bounds you need
{
    type Output = Self;

    fn mul(self, rhs: K) -> Self::Output {
        let mut vec = self.clone();
        for e in &mut vec.e {
            *e *= rhs;
        }
        vec
    }
}

impl<K> Vector<K> {
    pub fn new(e: Vec<K>) -> Self {
        Self { e }
    }

    pub fn display(&self)
    where
        K: Debug,
    {
        println!("{:?}", self.e);
    }

    // getter
    pub fn e(&self) -> &[K] {
        self.e.as_ref()
    }

    // mut getter
    pub fn set_e(&mut self, e: Vec<K>) {
        self.e = e;
    }

    // mut setter
    pub fn e_mut(&mut self) -> &mut Vec<K> {
        &mut self.e
    }

    pub fn size(&self) -> usize {
        self.e.len()
    }
    // MANDATORY -- ex00
    pub fn add_mut(&mut self, v: &Vector<K>)
    where
        K: AddAssign + Copy,
    {
        *self += v;
    }
    // MANDATORY -- ex00
    pub fn sub_mut(&mut self, v: &Vector<K>)
    where
        K: SubAssign + Copy,
    {
        *self -= v;
    }
    // MANDATORY -- ex00
    pub fn scl(&mut self, a: K)
    where
        K: MulAssign + Copy,
    {
        *self *= a;
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(value: Vec<K>) -> Self {
        Self { e: value }
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(value: [K; N]) -> Self {
        Self {
            e: Vec::from(value),
        }
    }
}

impl<K> From<&[K]> for Vector<K>
where
    K: Copy,
{
    fn from(value: &[K]) -> Self {
        Self {
            e: Vec::from(value),
        }
    }
}

impl<K: Debug> Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let buffer: String = self.e.iter().map(|col| format!("[{:?}]\n", col)).collect();
        write!(f, "{}", buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_util() {
        let mut u = Vector::from([1., 2.]);
        let v = Vector::from(vec![3., 4.]);
        v.display();
        let x = u.clone().sub(v);
        u.display();
        // let x = Vector::from(v);
        // v.display();  -- [ERROR] because what 'from()' does is shallow copying, 'clone' does deep copying
        u.display();
        x.display();

        let mut y = Vector::clone(&x); // 기본적으로 정의되는 함수를 사용하고 싶다면 Struct에 Derived 에 원하는 Trait을 추가해주면 된다.
        y.display();

        y += x;
        y.display();
        y -= u;
        y.display();
        // y *= &3.;
        // y.display();
        y *= 4.;
        y.display();
    }

    #[test]
    fn vector_scale() {
        let mut u = Vector::from([2., 3.]);
        u.scl(2.);
        assert_eq!(vec![4.0, 6.0], u.e);

        let mut u = Vector::from([0, 0]);
        u.scl(1);
        assert_eq!(vec![0, 0], u.e);

        let mut u = Vector::from([1, 0]);
        u.scl(1);
        assert_eq!(vec![1, 0], u.e);

        let mut u = Vector::from([1, 1]);
        u.scl(2);
        assert_eq!(vec![2, 2], u.e);

        let mut u = Vector::from([21, 21]);
        u.scl(2);
        assert_eq!(vec![42, 42], u.e);

        let mut u = Vector::from([42., 42.]);
        u.scl(0.5);
        assert_eq!(vec![21., 21.], u.e);
    }

    // #[test]
    // fn dot_basics() {
    //     let u = Vector::from([0., 0.]);
    //     let v = Vector::from([1., 1.]);
    //     assert_eq!(0.0, u.dot(v));
    //     let u = Vector::from([1., 1.]);
    //     let v = Vector::from([1., 1.]);
    //     assert_eq!(2., u.dot(v));
    //     let u = Vector::from([-1., 6.]);
    //     let v = Vector::from([3., 2.]);
    //     assert_eq!(9., u.dot(v));
    // }
    //
    // #[test]
    // fn dot_more() {
    //     let u = Vector::from([0, 0]);
    //     let v = Vector::from([0, 0]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from([1, 0]);
    //     let v = Vector::from([0, 0]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from([1, 0]);
    //     let v = Vector::from([1, 0]);
    //     assert_eq!(1, u.dot(v));
    //
    //     let u = Vector::from([1, 0]);
    //     let v = Vector::from([0, 1]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from([1, 1]);
    //     let v = Vector::from([1, 1]);
    //     assert_eq!(2, u.dot(v));
    //
    //     let u = Vector::from([4, 2]);
    //     let v = Vector::from([2, 1]);
    //     assert_eq!(10, u.dot(v));
    // }
    //
    // #[test]
    // fn norms_test_basics() {
    //     let u = Vector::from([0., 0., 0.]);
    //     assert_eq!(u.norm_1(), 0.0);
    //     assert_eq!(u.norm(), 0.0);
    //     assert_eq!(u.norm_inf(), 0.0);
    //
    //     let u = Vector::from([1., 2., 3.]);
    //     assert_eq!(u.norm_1(), 6.0);
    //     assert_eq!(u.norm(), 3.7416573);
    //     assert_eq!(u.norm_inf(), 3.);
    //
    //     let u = Vector::from([-1., -2.]);
    //     assert_eq!(u.norm_1(), 3.0);
    //     assert_eq!(u.norm(), 2.236067977);
    //     assert_eq!(u.norm_inf(), 2.);
    // }
    //
    // #[test]
    // fn norms_test_hards() {
    //     let u = Vector::from([0.]);
    //     assert_eq!(u.norm_1(), 0.);
    //     assert_eq!(u.norm(), 0.);
    //     assert_eq!(u.norm_inf(), 0.);
    //
    //     let u = Vector::from([1.]);
    //     assert_eq!(u.norm_1(), 1.);
    //     assert_eq!(u.norm(), 1.);
    //     assert_eq!(u.norm_inf(), 1.);
    //
    //     let u = Vector::from([0., 0.]);
    //     assert_eq!(u.norm_1(), 0.);
    //     assert_eq!(u.norm(), 0.);
    //     assert_eq!(u.norm_inf(), 0.);
    //
    //     let u = Vector::from([1., 0.]);
    //     assert_eq!(u.norm_1(), 1.);
    //     assert_eq!(u.norm(), 1.);
    //     assert_eq!(u.norm_inf(), 1.);
    //
    //     let u = Vector::from([2., 1.]);
    //     assert_eq!(u.norm_1(), 3.);
    //     assert_eq!(u.norm(), 2.236067977);
    //     assert_eq!(u.norm_inf(), 2.);
    //
    //     let u = Vector::from([4., 2.]);
    //     assert_eq!(u.norm_1(), 6.);
    //     assert_eq!(u.norm(), 4.472135955);
    //     assert_eq!(u.norm_inf(), 4.);
    //
    //     let u = Vector::from([-4., -2.]);
    //     assert_eq!(u.norm_1(), 6.);
    //     assert_eq!(u.norm(), 4.472135955);
    //     assert_eq!(u.norm_inf(), 4.);
    // }
}
