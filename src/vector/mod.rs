use core::fmt;
use num_complex::Complex;
use num_traits::Float;

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
        Ok(for (i, col) in self.e.iter().enumerate() {
            write!(f, "[{:?}]", col)?;
            if i < self.e.len() - 1 {
                write!(f, "\n")?;
            }
        })
    }
}

impl<K> AddAssign for Vector<K>
where
    K: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(
            self.e.len(),
            rhs.e.len(),
            "Vectors must be the same length for addition"
        );
        self.e
            .iter_mut()
            .zip(rhs.e.iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<K> SubAssign for Vector<K>
where
    K: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Vector<K>) {
        assert_eq!(
            self.e.len(),
            rhs.e.len(),
            "Vectors must be the same length for subtraction"
        );
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

impl<K> Add for Vector<K>
where
    K: AddAssign + Add<Output = K> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.e.len(),
            rhs.e.len(),
            "Vectors must be the same length for addition"
        );
        let mut vec = self.clone();
        for (p1, p2) in vec.e.iter_mut().zip(rhs.e.iter()) {
            *p1 += *p2;
        }
        vec
    }
}

impl<K> Sub for Vector<K>
where
    K: SubAssign + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.e.len(),
            rhs.e.len(),
            "Vectors must be the same length for subtraction"
        );
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
        println!("{}", self);
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
        *self += v.clone();
    }

    pub fn sub_mut(&mut self, v: &Vector<K>)
    where
        K: SubAssign + Copy,
    {
        *self -= v.clone();
    }

    pub fn scl(&mut self, a: K)
    where
        K: MulAssign + Copy,
    {
        *self *= a;
    }
    // END of ex00

    // MANDATORY -- ex03
    pub fn dot(&self, v: &Vector<K>) -> K
    where
        K: Default + Mul<Output = K> + AddAssign + Copy,
    {
        assert_eq!(self.size(), v.size(), "Vectors are not the same size");
        let mut value = K::default();
        self.e
            .iter()
            .zip(v.e.iter())
            .for_each(|(e1, e2)| value += *e1 * *e2);
        value
    }
    // END of ex03

    // MANDATORY -- ex04
    // NOTE -- Norms always return real numbers, evenfor complex-valued vectors.
    pub fn norm_1(&mut self) -> f32
    where
        K: Float,
        f32: AddAssign<K>,
    {
        let mut result = f32::default();
        for p in &self.e {
            result += p.abs();
        }
        result
    }

    pub fn norm(&mut self) -> f32
    where
        K: Float,
        f32: AddAssign<K>,
    {
        let mut result = f32::default();
        for p in &self.e {
            result += p.powf(K::from(2.0).unwrap());
        }
        result.sqrt()
    }

    pub fn norm_inf(&mut self) -> f32
    where
        K: Float,
        f32: AddAssign<K> + PartialOrd<K> + Copy,
    {
        let mut result = f32::min_value();
        for p in &self.e {
            if result < p.abs() {
                result = p.abs().to_f32().unwrap();
            }
        }
        result
    }
    // END of ex04
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_utils() {
        let u = Vector::from([1., 2., 3., 4.]);

        u.display();
        println!("{}", u);
        println!("{:?}", u);
        println!("{:#?}", u);

        assert_eq!(4, u.size());
    }

    #[test]
    fn vector_add() {
        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u.add_mut(&v);
        assert_eq!(Vec::from([8.0, 6.0]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u = u.add(v);
        assert_eq!(Vec::from([8.0, 6.0]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u = u + v;
        assert_eq!(Vec::from([8.0, 6.0]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u += v;
        assert_eq!(Vec::from([8.0, 6.0]), u.e);
    }

    #[test]
    fn vector_sub() {
        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u.sub_mut(&v);
        assert_eq!(Vec::from([-6., -2.]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u = u.sub(v);
        assert_eq!(Vec::from([-6., -2.]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u = u - v;
        assert_eq!(Vec::from([-6., -2.]), u.e);

        let mut u = Vector::from([1., 2.]);
        let v = Vector::from([7., 4.]);
        u -= v;
        assert_eq!(Vec::from([-6., -2.]), u.e);
    }

    #[test]
    fn vector_scale() {
        let mut u = Vector::from([1., 1.]);
        u.scl(42.);
        assert_eq!(Vec::from([42., 42.]), u.e);

        let mut u = Vector::from([1., 1.]);
        u *= 42.;
        assert_eq!(Vec::from([42., 42.]), u.e);

        let mut u = Vector::from([1., 1.]);
        u = u * 42.;
        assert_eq!(Vec::from([42., 42.]), u.e);
    }

    #[test]
    fn dot_basics() {
        let u = Vector::from([0., 0.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(0.0, u.dot(&v));
        let u = Vector::from([1., 1.]);
        let v = Vector::from([1., 1.]);
        assert_eq!(2., u.dot(&v));
        let u = Vector::from([-1., 6.]);
        let v = Vector::from([3., 2.]);
        assert_eq!(9., u.dot(&v));
    }

    #[test]
    fn dot_more() {
        let u = Vector::from([0, 0]);
        let v = Vector::from([0, 0]);
        assert_eq!(0, u.dot(&v));

        let u = Vector::from([1, 0]);
        let v = Vector::from([0, 0]);
        assert_eq!(0, u.dot(&v));

        let u = Vector::from([1, 0]);
        let v = Vector::from([1, 0]);
        assert_eq!(1, u.dot(&v));

        let u = Vector::from([1, 0]);
        let v = Vector::from([0, 1]);
        assert_eq!(0, u.dot(&v));

        let u = Vector::from([1, 1]);
        let v = Vector::from([1, 1]);
        assert_eq!(2, u.dot(&v));

        let u = Vector::from([4, 2]);
        let v = Vector::from([2, 1]);
        assert_eq!(10, u.dot(&v));
    }

    #[test]
    fn norms_test_basics() {
        let mut u = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.0);
        assert_eq!(u.norm(), 0.0);
        assert_eq!(u.norm_inf(), 0.0);

        let mut u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
        assert_eq!(u.norm(), 3.7416575);
        assert_eq!(u.norm_inf(), 3.);

        let mut u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
        assert_eq!(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.);
    }

    #[test]
    fn norms_test_hards() {
        let mut u = Vector::from([0.]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);

        let mut u = Vector::from([1.]);
        assert_eq!(u.norm_1(), 1.);
        assert_eq!(u.norm(), 1.);
        assert_eq!(u.norm_inf(), 1.);

        let mut u = Vector::from([0., 0.]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);

        let mut u = Vector::from([1., 0.]);
        assert_eq!(u.norm_1(), 1.);
        assert_eq!(u.norm(), 1.);
        assert_eq!(u.norm_inf(), 1.);

        let mut u = Vector::from([2., 1.]);
        assert_eq!(u.norm_1(), 3.);
        assert_eq!(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.);

        let mut u = Vector::from([4., 2.]);
        assert_eq!(u.norm_1(), 6.);
        assert_eq!(u.norm(), 4.472135955);
        assert_eq!(u.norm_inf(), 4.);

        let mut u = Vector::from([-4., -2.]);
        assert_eq!(u.norm_1(), 6.);
        assert_eq!(u.norm(), 4.472135955);
        assert_eq!(u.norm_inf(), 4.);
    }
}
