//! Contains the implementation of `MatrixExt` for the standard 2D array `[[T; N]; M]`.

use crate::{MatrixExt, MatrixMutExt};
use crate::req::*;
use crate::strategies::Transpose;

impl<T, const N: usize, const M: usize> MatrixExt for [[T; N]; M]
{
    type Element = T;
    
    fn num_rows(&self) -> usize { 
        let len = self.len();
        if len != 0 && self[0].len() != 0 {
            len
        } 
        else {
            0
        }
    }
    fn num_cols(&self) -> usize { 
        if self.len() == 0 { 
            0 
        } else {
            self[0].len() 
        }
    }
    fn get(&self, i: usize, j: usize) -> Option<&Self::Element> {
        self.as_slice().get(i)?.as_slice().get(j)
    }
}

impl<T, const N: usize, const M: usize> MatrixMutExt for [[T; N]; M] {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Element> {
        self.as_mut_slice().get_mut(i)?.as_mut_slice().get_mut(j)
    }
}

impl<T: Default, const M: usize, const N: usize> TransformStrategy<[[T; M]; N]> for Transpose {
    type Output = [[T; N]; M];
    fn out_of(&self, m: [[T; M]; N]) -> Self::Output {
        let mut t: [[T; N]; M] = ::core::array::from_fn(|_|
            ::core::array::from_fn(|_| 
                T::default()
            )
        );

        for (i, row) in IntoIterator::into_iter(m).enumerate() {
            for (j, elem) in IntoIterator::into_iter(row).enumerate() {
                *t.get_mut(j, i).unwrap() = elem;
            }
        }
        t
    }
}

impl<T, const N: usize> InPlace<[[T; N]; N]> for Transpose {
    fn in_place(&self, m: &mut [[T; N]; N]) {
        for i in 0..N {
            for j in 0..i {
                m.swap((i, j), (j, i));
            }
        }
    }
}
