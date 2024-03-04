//! Traits required for performing operations on `MatrixExt` structures are all packed here.

use crate::{ MatrixExt, MatrixMutExt };

/// Provides a method for exchanging the dimensions of a matrix.
///
/// This trait is intended for some matrix transformations.
pub trait SwapsDimensions: MatrixMutExt {
    /// After calling this function, `.num_rows()` must return value of `.num_cols()` and vice-versa.
    fn swap_dimensions(&mut self);
}


/// Create a `MatrixExt` from an iterator. 
pub trait MatrixExtFromIter<A> {
    fn from_iter<T>(into_iter: T, columns: usize) -> Self
        where T: IntoIterator<Item = A>;
}


/// A trait for in-place modification of matrices.
pub trait InPlace<M: MatrixExt>: Sized {
    fn in_place(&self, m: &mut M);
}


/// A Strategy trait for matrix transformation.
pub trait TransformStrategy<M: MatrixExt> {
    type Output;
    // By chosing to consume the matrix in order to obtain its transformation,
    // we allow using either an in-place modification of the matrix or the creation 
    // of a new matrix, where one can be more efficient than the other. 
    fn out_of(&self, m: M) -> Self::Output;
}

/// A Strategy trait for access to matrix elements.
pub trait AccessStrategy<M: MatrixExt> {
    /// Gives the location (if it exists) of the element in a matrix that 
    /// should match the indexes (subscripts) provided according to the access strategy.
    ///
    /// A return of `None` is interpereted an out of bound access.
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)>;
    
    /// Gives the number of rows for the `Access` matrix defined by the `AccessStrategy`.
    fn nrows(&self, m: &M) -> usize;
    
    /// Gives the number of columns for the `Access` matrix defined by the `AccessStrategy`.
    fn ncols(&self, m: &M) -> usize;
}
