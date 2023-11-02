// use std::ops::{AddAssign, MulAssign};

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};

pub use crate::matrix::Matrix;
pub use crate::vector::Vector;

mod matrix;
mod vector;

fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: MulAssign + AddAssign + Copy + Clone + Default,
{
    let mut result = Vector::from(vec![K::default(); u[0].size()]);
    // &vec![K::default(); u[0].size()] === vec![K::default(); u[0].size()].as_slice()
    u.to_vec()
        .iter_mut()
        .zip(coefs.iter())
        .for_each(|(v, &coef)| {
            v.scl(coef);
            result.add(v);
        });
    result
}

// linear interpolation
fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: Add<Output = V> + Sub<Output = V> + Mul<f32, Output = V>,
{
    u * (1.0 - t) + v * t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_linear_combination() {
        let e1 = Vector::from([1., 0., 0.]);
        let e2 = Vector::from([0., 1., 0.]);
        let e3 = Vector::from([0., 0., 1.]);
        let v1 = Vector::from([1., 2., 3.]);
        let v2 = Vector::from([0., 10., -100.]);
        println!(
            "{:?}",
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
                Matrix::from([[20., 10.], [30., 40.]]),
                0.3
            )
        );
    }
}

fn main() {}
