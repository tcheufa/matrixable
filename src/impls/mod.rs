use crate::traits::*;

use std::ops::{Deref, DerefMut};

impl<T> Swap for [T] { 
    fn swap(&mut self, i:usize, j:usize) {
        self.swap(i, j);
    } 
}

impl<T, U: Collection<T> + DerefMut<Target = [T]>> Swap for U { 
    fn swap(&mut self, i:usize, j:usize) {
        self.as_mut().swap(i, j);
    } 
}

impl<T> Collection<T> for [T] {  }
impl<T, S: Deref<Target = [T]>> Collection<T> for S {  }

/* Not a good idea at all.
impl<T> Matrix for [T] {
    type Element = T;
    
    fn num_rows(&self) -> usize { 1 }

    fn num_cols(&self) -> usize { self.len() }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        self.get(column) 
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> {
        self.get_mut(column)
    } 
}

impl<T, S: Deref<Target = [T]>> Matrix for S {
    type Element = T;
    
    fn num_rows(&self) -> usize { 1 }

    fn num_cols(&self) -> usize { self.len() }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let n = self.index_from(row, column);
        self.get(n) 
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> {
        let n = self.index_from(row, column);
        self.get_mut(n)
    } 
}*/

// /// Indexation follows the row major order by default.
// impl<M: Matrix>  std::ops::Index<usize<M>> for M {
//     type Output = <M as Matrix>::Element;
//     
//     fn index(&self, index: usize) -> &Self::Output {
//         self.row(index);
//     }
// }


// /// Indexation follows the row major order by default.
// impl<E, D> std::ops::IndexMut<usize> for Matrix<Data = D, Element = E>
// where
//     D: std::ops::IndexMut<std::ops::Range<usize>, Output = D>
//       +std::ops::IndexMut<usize, Output = E>
// {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let cols = self.num_cols();
//         let start = cols * index;
//         let end = start + cols;
//         &mut self.data_mut()[start..end]
//     }
// }

