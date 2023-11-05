// use std::ops::{AddAssign, MulAssign};

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

use num_traits::Float;

pub use crate::matrix::Matrix;
pub use crate::vector::Vector;

mod matrix;
mod vector;

// MANDATORY -- ex01
#[allow(dead_code)]
fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: MulAssign + AddAssign + Copy + Clone + Default,
{
    assert_eq!(
        u.len(),
        coefs.len(),
        "Lengths of u and coefs must be the same"
    );

    let mut result = Vector::from(vec![K::default(); u[0].size()]);
    u.to_vec()
        .iter_mut()
        .zip(coefs.iter())
        .for_each(|(v, &coef)| {
            v.scl(coef);
            result.add_mut(v);
        });
    result
}
// END of ex01

// #[allow(dead_code)]
// fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Result<Vector<K>, &'static str>
// where
//     K: MulAssign + AddAssign + Copy + Clone + Default,
// {
//     if u.len() != coefs.len() {
//         return Err("Lengths of u and coefs must be the same");
//     }

//     let mut result = Vector::from(vec![K::default(); u[0].size()]);
//     u.to_vec()
//         .iter_mut()
//         .zip(coefs.iter())
//         .for_each(|(v, &coef)| {
//             v.scl(coef);
//             result.add_mut(v);
//         });
//     Ok(result)
// }

// MANDATORY -- ex02
// linear interpolation
#[allow(dead_code)]
fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> + Sub<Output = V> + Mul<f32, Output = V>,
{
    let factor = 1.0 - t;
    u * factor + v * t
}
// END of ex02

// MANDATORY -- ex05
// NOTE -- The usagge of the standard library's cos function is forbidden.
#[allow(dead_code)]
fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Default + Mul<Output = K> + AddAssign + Copy + Float,
    f32: AddAssign<K>,
{
    u.dot(v).to_f32().unwrap() / (u.clone().norm() * v.clone().norm())
}
// END of ex05

#[cfg(test)]
mod tests {
    use std::f32::EPSILON;

    use super::*;

    #[test]
    pub fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        println!(
            "{:#?}",
            linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5])
        );
        // [10.]
        // [-2.]
        // [0.5]
        println!(
            "{:?}",
            linear_combination::<f32>(&[v1.clone(), v2.clone()], &[10., -2.])
        );
        // [10.]
        // [0.]
        // [230.]

        println!("{}", lerp(21., 42., 0.3));
        println!(
            "{}",
            lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3)
        );

        println!(
            "{}",
            lerp(
                Matrix::from([[2., 1.], [4., 2.]]),
                Matrix::from([[20., 1.], [30., 1.]]),
                0.3
            )
        );
    }

    #[test]
    fn cosine_tests() {
        let u = Vector::from(vec![1., 0.]);
        let v = Vector::from(vec![1., 0.]);
        assert_eq!((angle_cos(&u, &v) - 1.0).abs() < EPSILON * 100., true);
        // 1.0
        let u = Vector::from(vec![1., 0.]);
        let v = Vector::from(vec![0., 1.]);
        assert_eq!((angle_cos(&u, &v) - 0.0).abs() < EPSILON * 100., true);
        // 0.0
        let u = Vector::from(vec![-1., 1.]);
        let v = Vector::from(vec![1., -1.]);
        assert_eq!((angle_cos(&u, &v) + 1.0).abs() < EPSILON * 100., true);
        // -1.0
        let u = Vector::from(vec![2., 1.]);
        let v = Vector::from(vec![4., 2.]);
        assert_eq!((angle_cos(&u, &v) - 1.0).abs() < EPSILON * 100., true);
        // 1.0
        let u = Vector::from(vec![1., 2., 3.]);
        let v = Vector::from(vec![4., 5., 6.]);
        assert_eq!(
            (angle_cos(&u, &v) - 0.974631846).abs() < EPSILON * 100.,
            true
        );
        // 0.974631846

        //// undefined
        // let u = Vector::from(vec![1., 2.]);
        // let v = Vector::from(vec![4., 5., 6.]);
        // println!("{}", angle_cos(&u, &v)); // PANIC

        //// NaN
        // let u = Vector::from(vec![]);
        // let v = Vector::from(vec![4., 5., 6.]);
        // println!("{}", angle_cos(&u, &v)); // PANIC

        // let u = Vector::from(vec![1.]);
        // let v = Vector::from(vec![]);
        // println!("{}", angle_cos(&u, &v)); // PANIC
    }
}

fn main() {}
