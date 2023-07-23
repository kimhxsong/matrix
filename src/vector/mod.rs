use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Index, IndexMut, MulAssign, SubAssign};

#[derive(Clone, Debug)]
pub struct Vector<K> {
    pub elements: Vec<K>,
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn add(&mut self, v: &Vector<K>)
        where
            K: AddAssign + Copy,
    {
        assert_eq!(self.elements.len(), v.elements.len());
        self.elements
            .iter_mut()
            .zip(v.elements.iter())
            .for_each(|(a, b)| *a += *b);
    }

    pub fn sub(&mut self, v: &Vector<K>)
        where
            K: SubAssign + Copy,
    {
        assert_eq!(self.elements.len(), v.elements.len());
        self.elements
            .iter_mut()
            .zip(v.elements.iter())
            .for_each(|(a, b)| *a -= *b);
    }

    pub fn scl(&mut self, a: K)
        where
            K: MulAssign + Copy,
    {
        self.elements.iter_mut().for_each(|x| *x *= a);
    }

    pub fn display(&self)
        where
            K: Debug,
    {
        println!("{:?}", self.elements);
    }
}

impl<K: Clone> From<Vec<K>> for Vector<K> {
    fn from(vec: Vec<K>) -> Self {
        Vector { elements: vec }
    }
}

impl<K> Vector<K> {
    pub fn from(array: &[K]) -> Self
        where
            K: Clone,
    {
        Self {
            elements: Vec::from(array),
        }
    }
}

impl<K: Debug> Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_util() {
        let vec = vec![1, 2, 3];
        let u: Vector<_> = vec.into(); // From<Vec<K>>

        let arr = &[1, 2, 3];
        let v = Vector::from(arr); // from(&[K])

        u.display();
        println!("{}", u); // [2.0, 3.0]
        println!("{:?}", u); // Vector { elements: [2.0, 3.0] }

        v.display();
        println!("{}", u); // [2.0, 3.0]
        println!("{:?}", u); // Vector { elements: [2.0, 3.0] }
    }

    #[test]
    fn vector_add() {
        let mut u = Vector::from(&[2., 3.]);
        let v = Vector::from(&[5., 7.]);

        u.add(&v);
        assert_eq!(vec![7.0, 10.0], u.elements);

        let mut u = Vector::from(&[0, 0]);
        let v = Vector::from(&[0, 0]);
        u.add(&v);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[1, 0]);
        let v = Vector::from(&[0, 1]);
        u.add(&v);
        assert_eq!(vec![1, 1], u.elements);

        let mut u = Vector::from(&[1, 1]);
        let v = Vector::from(&[1, 1]);
        u.add(&v);
        assert_eq!(vec![2, 2], u.elements);

        let mut u = Vector::from(&[21, 21]);
        let v = Vector::from(&[21, 21]);
        u.add(&v);
        assert_eq!(vec![42, 42], u.elements);

        let mut u = Vector::from(&[-21, 21]);
        let v = Vector::from(&[21, -21]);
        u.add(&v);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v = Vector::from(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        u.add(&v);
        assert_eq!(vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], u.elements);
    }

    #[test]
    fn vector_sub() {
        let mut u = Vector::from(&[2., 3.]);
        let v = Vector::from(&[5., 7.]);
        u.sub(&v);
        assert_eq!(vec![-3.0, -4.0], u.elements);

        let mut u = Vector::from(&[0, 0]);
        let v = Vector::from(&[0, 0]);
        u.sub(&v);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[1, 0]);
        let v = Vector::from(&[0, 1]);
        u.sub(&v);
        assert_eq!(vec![1, -1], u.elements);

        let mut u = Vector::from(&[1, 1]);
        let v = Vector::from(&[1, 1]);
        u.sub(&v);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[21, 21]);
        let v = Vector::from(&[21, 21]);
        u.sub(&v);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[-21, 21]);
        let v = Vector::from(&[21, -21]);
        u.sub(&v);
        assert_eq!(vec![-42, 42], u.elements);

        let mut u = Vector::from(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let v = Vector::from(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
        u.sub(&v);
        assert_eq!(vec![-9, -7, -5, -3, -1, 1, 3, 5, 7, 9], u.elements);
    }

    #[test]
    fn vector_scale() {
        let mut u = Vector::from(&[2., 3.]);
        u.scl(2.);
        assert_eq!(vec![4.0, 6.0], u.elements);

        let mut u = Vector::from(&[0, 0]);
        u.scl(1);
        assert_eq!(vec![0, 0], u.elements);

        let mut u = Vector::from(&[1, 0]);
        u.scl(1);
        assert_eq!(vec![1, 0], u.elements);

        let mut u = Vector::from(&[1, 1]);
        u.scl(2);
        assert_eq!(vec![2, 2], u.elements);

        let mut u = Vector::from(&[21, 21]);
        u.scl(2);
        assert_eq!(vec![42, 42], u.elements);

        let mut u = Vector::from(&[42., 42.]);
        u.scl(0.5);
        assert_eq!(vec![21., 21.], u.elements);
    }

    // #[test]
    // fn dot_basics() {
    //     let u = Vector::from(&[0., 0.]);
    //     let v = Vector::from(&[1., 1.]);
    //     assert_eq!(0.0, u.dot(v));
    //     let u = Vector::from(&[1., 1.]);
    //     let v = Vector::from(&[1., 1.]);
    //     assert_eq!(2., u.dot(v));
    //     let u = Vector::from(&[-1., 6.]);
    //     let v = Vector::from(&[3., 2.]);
    //     assert_eq!(9., u.dot(v));
    // }
    //
    // #[test]
    // fn dot_more() {
    //     let u = Vector::from(&[0, 0]);
    //     let v = Vector::from(&[0, 0]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from(&[1, 0]);
    //     let v = Vector::from(&[0, 0]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from(&[1, 0]);
    //     let v = Vector::from(&[1, 0]);
    //     assert_eq!(1, u.dot(v));
    //
    //     let u = Vector::from(&[1, 0]);
    //     let v = Vector::from(&[0, 1]);
    //     assert_eq!(0, u.dot(v));
    //
    //     let u = Vector::from(&[1, 1]);
    //     let v = Vector::from(&[1, 1]);
    //     assert_eq!(2, u.dot(v));
    //
    //     let u = Vector::from(&[4, 2]);
    //     let v = Vector::from(&[2, 1]);
    //     assert_eq!(10, u.dot(v));
    // }
    //
    // #[test]
    // fn norms_test_basics() {
    //     let u = Vector::from(&[0., 0., 0.]);
    //     assert_eq!(u.norm_1(), 0.0);
    //     assert_eq!(u.norm(), 0.0);
    //     assert_eq!(u.norm_inf(), 0.0);
    //
    //     let u = Vector::from(&[1., 2., 3.]);
    //     assert_eq!(u.norm_1(), 6.0);
    //     assert_eq!(u.norm(), 3.7416573);
    //     assert_eq!(u.norm_inf(), 3.);
    //
    //     let u = Vector::from(&[-1., -2.]);
    //     assert_eq!(u.norm_1(), 3.0);
    //     assert_eq!(u.norm(), 2.236067977);
    //     assert_eq!(u.norm_inf(), 2.);
    // }
    //
    // #[test]
    // fn norms_test_hards() {
    //     let u = Vector::from(&[0.]);
    //     assert_eq!(u.norm_1(), 0.);
    //     assert_eq!(u.norm(), 0.);
    //     assert_eq!(u.norm_inf(), 0.);
    //
    //     let u = Vector::from(&[1.]);
    //     assert_eq!(u.norm_1(), 1.);
    //     assert_eq!(u.norm(), 1.);
    //     assert_eq!(u.norm_inf(), 1.);
    //
    //     let u = Vector::from(&[0., 0.]);
    //     assert_eq!(u.norm_1(), 0.);
    //     assert_eq!(u.norm(), 0.);
    //     assert_eq!(u.norm_inf(), 0.);
    //
    //     let u = Vector::from(&[1., 0.]);
    //     assert_eq!(u.norm_1(), 1.);
    //     assert_eq!(u.norm(), 1.);
    //     assert_eq!(u.norm_inf(), 1.);
    //
    //     let u = Vector::from(&[2., 1.]);
    //     assert_eq!(u.norm_1(), 3.);
    //     assert_eq!(u.norm(), 2.236067977);
    //     assert_eq!(u.norm_inf(), 2.);
    //
    //     let u = Vector::from(&[4., 2.]);
    //     assert_eq!(u.norm_1(), 6.);
    //     assert_eq!(u.norm(), 4.472135955);
    //     assert_eq!(u.norm_inf(), 4.);
    //
    //     let u = Vector::from(&[-4., -2.]);
    //     assert_eq!(u.norm_1(), 6.);
    //     assert_eq!(u.norm(), 4.472135955);
    //     assert_eq!(u.norm_inf(), 4.);
    // }
}
