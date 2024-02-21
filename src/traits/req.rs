//! Traits other than the `Matrix` trait are all packed here.

use crate::traits::Matrix;

/// Provides a method for exchanging the dimensions of a matrix.
///
/// This can be helpful in matrix transformation.
pub trait SwapDimensions: Matrix {
    /// After calling this function, `.num_rows()` must become `.num_cols()` and vice-versa.
    fn swap_dimensions(&mut self);
}

/// A `Matrix` implementing this trait can be created from an iterator by specifying the number of rows and columns of the newly created matrix. 
pub trait MatrixFromIter<A> {
    fn from_iter<T>(into_iter: T, rows: usize, columns: usize) -> Self
        where T: IntoIterator<Item = A>;
}

/// A Strategy trait for matrix transformation.
pub trait TransformStrategy<M: Matrix>: AccessStrategy<M> {
    // By chosing to consume the matrix in order to obtain its transformation,
    // we allow either using either an in-place modification of the matrix or the creation of a 
    // new matrix, where one of them can be more efficient than the other. 
    // This could not be achieved by taking &mut M as parameter and returning nothing.
    fn out_of(&self, m: M) -> M; // becomes 
}

/// A Strategy trait for access to matrix elements.
pub trait AccessStrategy<M: Matrix> {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)>;
    
    fn new_nrows(&self, m: &M) -> usize;
    fn new_ncols(&self, m: &M) -> usize;
}
