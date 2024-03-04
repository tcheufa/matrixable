//! Contains the implementation of `Matrix` for the standard 2D array `[[T; N]; M]`.

use crate::{Matrix, MatrixMut};
use crate::req::*;
use crate::strategies::Transpose;

impl<T, const N: usize, const M: usize> Matrix for [[T; N]; M]
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

impl<T, const N: usize, const M: usize> MatrixMut for [[T; N]; M] {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Self::Element> {
        self.as_mut_slice().get_mut(i)?.as_mut_slice().get_mut(j)
    }
}

impl<T: Default + Copy, const N: usize, const M: usize> TransformStrategy<[[T; N]; M]> for Transpose { 
    type Output = [[T; M]; N];
    fn out_of(&self, m: [[T; N]; M]) -> Self::Output {
        let mut t = [[T::default(); M]; N];
        t.iter_mut().zip(m.access(Transpose).iter())
                    .for_each(|(x, y)| *x = *y);
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

