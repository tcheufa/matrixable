//! This module provides a growable and generic matrix that implements the `Matrix` trait.

use crate::*;
use crate::traits::{ Matrix, MatrixMut, SwapDimensions };

/// A growable and generic matrix.
///
/// 
/// TODO:
/// - Test methods that have not been
/// - Look for a better matrix multiplication optimization
/// - Search for matrix operations
/// - Test Trait Implementations
/// - Change `ops.rs` filename into `trait.rs` and copy all trait impl in that file
/// - Add modules inside `trait.rs` like `ops.rs`, `convert.rs`, `custom.rs`...
/// - Review Extend trait implementation design.                                            :OK:
/// - Review the design decision of not allowing empty matrices
/// - Try an `into_diagos()` method to convert a matrix into a its diagonals.               :OK:
/// - Implement the `Growable` trait for MatrixView;

#[derive(Clone, Default, Debug)]
pub struct MatrixView<T> {
    pub(crate) d: Vec<T>,
    pub(crate) c: usize,
}
impl<T> MatrixView<T> {
    /// Creates a new matrix filled with `init_val`.
    ///
    /// # Panics
    /// Panics if  `rows == 0 ` or `columns == 0`.
    pub fn with_init(init_val: T, rows: usize, columns: usize) -> Self
    where
        T: Clone,
    {
        panic_if_bad_size(rows, columns);

        MatrixView {
            d: vec![init_val; columns * rows],
            c: columns,
        }
    }

    /// Creates a new empty_matrix.
    ///
    /// # Panics
    /// Panics if  `rows == 0 ` or `columns == 0`.
    pub fn with_capacity(rows: usize, columns: usize) -> Self {
        panic_if_bad_size(rows, columns);

        Self {
            d: Vec::with_capacity(columns * rows),
            c: columns,
        }
    }

    /// Creates a matrix from a vector.
    ///
    /// # Panics
    /// Panics if `v.len() != rows * cols`, if  `rows == 0 ` or if `columns == 0`.
    pub fn new(mut data: Vec<T>, rows: usize, columns: usize) -> Self {
        panic_if_bad_size(rows, columns);

        // Avoid unnecessary allocation
        data.shrink_to_fit();

        println!("len = {}, capacity = {}", data.len(), data.capacity());

        if data.len() != rows * columns {
            panic!(
                "vec length ({}) does not fit matrix size ({}).",
                data.capacity(),
                rows * columns
            )
        }

        Self { d: data, c: columns }
    }
    
    fn into_vec(self) -> Vec<T> {  self.d  }
    
    pub fn data(&self) -> &Vec<T> { &self.d }
    
    pub fn data_mut(&mut self) -> &mut [T] { &mut self.d }
    
    #[inline(always)]
    pub fn is_empty(&self) -> bool { self.d.is_empty() }
 }
 
 impl<T> Into<Vec<T>> for MatrixView<T> {
    fn into(self) -> Vec<T> { self.into_vec() }
 }
 
 
impl<T> Matrix for MatrixView<T> {
    type Element = T;
 
    #[inline(always)]
    fn num_rows(&self) -> usize {  
        if self.is_empty() {
            self.d.capacity() / self.c
        }
        else {
            self.d.len() / self.c  
        }
    }

    #[inline(always)]
    fn num_cols(&self) -> usize {  self.c  }
    
    fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < self.num_rows() && j < self.c {
            return self.d.get(self.index_from((i, j)));
        }
        None
    }
}
impl<T> MatrixMut for MatrixView<T> {
    fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i < self.num_rows() && j < self.c {
            let n = self.index_from((i, j));
            return self.d.get_mut(n);
        }
        None
    }
    
    fn swap (&mut self, a: (usize, usize), b: (usize, usize)) {
        let a = self.index_from(a);
        let b = self.index_from(b);
        self.d.swap(a, b); // Data must have the method swap.
    }
}

impl<T> SwapDimensions for MatrixView<T> {
    fn swap_dimensions(&mut self) {
        self.c = self.num_rows();
    }
}


// /// Indexation follows the row major order by default.
// impl<T> std::ops::Index<usize> for MatrixView<T> {
//     type Output = [T];
//     fn index(&self, index: usize) -> &Self::Output {
//         let start = self.c * index;
//         let end = start + self.c;
//         &self.d[start..end]
//     }
// }
// 
// 
// /// Indexation follows the row major order by default.
// impl<T> std::ops::IndexMut<usize> for MatrixView<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let start = self.c * index;
//         let end = start + self.c;
//         &mut self.d[start..end]
//     }
// }


/// Simple and safe type conversions that may fail in a controlled way under some circumstances.
///
/// For a conversion into a matrix that can fail,
/// use this instead of the original `TryFrom` trait from the standard library.
trait TryFrom<T>: Sized {
    type Error;
    fn try_from(source: T) -> Result<Self, Self::Error>;
}


impl<C, T> crate::view::TryFrom<C> for MatrixView<T>
where
    C: IntoIterator,

<C as IntoIterator>::IntoIter: ExactSizeIterator,

<C as IntoIterator>::Item: IntoIterator<Item = T>,

<<<C as IntoIterator>
  ::IntoIter as Iterator>
    ::Item as IntoIterator>
    ::IntoIter: ExactSizeIterator,
{
    type Error = &'static str;

    fn try_from(source: C) -> Result<Self, Self::Error> {
        let mut source = source.into_iter();

        if source.len() == 0 { 
            return Err("data should not be empty") 
        }

        let first = source.nth(0).expect("source size must be >= 1").into_iter();
        let cols = first.len();

        let mut data = Vec::from_iter(first);
        for x in source {
            let x = x.into_iter();
            if x.len() != cols {
                return Err("rows must have the same length")
            }
            data.extend(x);
        }

        Ok(Self { d: data, c: cols })
    }
}

impl<T: Clone, const M: usize, const N: usize> From<[[T; N]; M]> for MatrixView<T> {
    fn from(source: [[T; N]; M]) -> Self {
        let mut d = Vec::with_capacity(N * M);
        for s in source {
            d.extend(s);
        }
        Self { d, c: N}
    }
}
