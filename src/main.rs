use std::ops::{AddAssign, MulAssign};

pub use crate::vector::Vector;

mod vector;
mod matrix;

fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
    where K: MulAssign<K> + AddAssign<K> + Copy + Clone + Default,
{
    let mut result = Vector::from(&vec![K::default(); u[0].size()]);
    // &vec![K::default(); u[0].size()] === vec![K::default(); u[0].size()].as_slice()
    u.to_vec().iter_mut().zip(coefs.iter()).for_each(|(v, &coef)| {
        v.scl(coef);
        result.add(v);
    });
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_linear_combination()
    {
        let e1 = Vector::from(&[1., 0., 0.]);
        let e2 = Vector::from(&[0., 1., 0.]);
        let e3 = Vector::from(&[0., 0., 1.]);
        let v1 = Vector::from(&[1., 2., 3.]);
        let v2 = Vector::from(&[0., 10., -100.]);
        println!("{}", linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]));
        // [10.]
        // [-2.]
        // [0.5]
        println!("{}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
        // [10.]
        // [0.]
        // [230.]
    }
}

fn main() {}