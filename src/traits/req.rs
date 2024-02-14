//! Traits other than the `Matrix` trait are all packed here.

use crate::traits::{Matrix};


/// Provides a method for exchanging the dimensions of a matrix.
///
/// This can be helpful in matrix transformation.
pub trait SwapDimensions: Matrix {
    /// After calling this function, `.num_rows()` must become `.num_cols()` and vice-versa.
    fn swap_dimensions(&mut self);
}


/// A Strategy for matrix transformation.
pub trait TransformStrategy<M: Matrix> {
    //fn clone(m: &M) -> M where &M::Element: Clone;
    
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)>;
    // By chosing to consume the matrix in order to obtain its transformation,
    // we allow either using either an in-place modification of the matrix or the creation of a 
    // new matrix, where one of them can be more efficient than the other. 
    // This could not be achieved by taking &mut M as parameter and returning nothing.
    fn out_of(m: M) -> M; // becomes 
    //fn in_place(&mut m);
    // from(m: M) -> M;
}

/// A Strategy for matrix elements' access.
pub trait AccessStrategy<M: Matrix> {
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)>;
    
    fn access_nth(m: &M, n: usize) -> Option<usize> {
        let (i, j) = m.indexes_from(n);
        Some(m.index_from(Self::access(m, i, j)?))
    }
}
