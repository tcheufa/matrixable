//! Traits required for performing operations on `MatrixExt` structures are all packed here.


use crate::{ MatrixExt, MatrixMutExt };

/// Exchange matrix dimensions.
///
/// This trait is intended for some matrix transformations.
pub trait SwapsDimensions: MatrixMutExt {
    /// After calling this function, `.num_rows()` must return value of `.num_cols()` and vice-versa.
    fn swap_dimensions(&mut self);
}


/// Create a matrix from an iterator.
pub trait MatrixExtFromIter<A> {
    fn from_iter<I>(into_iter: I, columns: usize) -> Self
        where I: IntoIterator<Item = A>,
             <I as IntoIterator>::IntoIter: ExactSizeIterator;
}


/// A trait for in-place modification of matrices.
/// The following example shows the implementation of the `SortBy` strategy used in this crate.
///
/// # Example
/// ```
/// use matrixable::req::InPlace;
/// use matrixable::MatrixMutExt;
///
/// pub struct SortBy<T> (fn(&T, &T) -> bool);
///
/// impl<M: MatrixMutExt> InPlace<M> for SortBy<M::Element> {
///     fn in_place(&self, m: &mut M) {
///         let mut im;
///         let mut min_or_max;
///         let mut cmp;
///
///         for i in 0..(m.size() - 1) {
///             im = i;
///             min_or_max = m.get_nth(i).unwrap();
///             for j in (i+1)..m.size() {
///                 cmp = m.get_nth(j).unwrap();
///                 if !(self.0)(min_or_max, cmp) {
///                       im = j;
///                       min_or_max = cmp;
///                 }
///             }
///             m.swapn(im, i);
///         }
///     }
/// } 
///
/// let mut m = [ 
///     [4,  5,  6],
///     [9,  1, 20],
///     [4, 12, -1]
/// ];
///
/// m.in_place(SortBy(|a, b| a < b));
/// 
/// assert_eq!(m, [
///     [-1,  1,  4],
///     [ 4,  5,  6],
///     [ 9, 12, 20]
/// ]);
/// ``` 
pub trait InPlace<M: MatrixMutExt>: Sized {
    fn in_place(&self, m: &mut M);
}


/// A strategy trait for matrix transformation.
pub trait TransformStrategy<M: MatrixExt> {
    type Output;
    // By chosing to consume the matrix in order to obtain its transformation,
    // we allow using either an in-place modification of the matrix or the creation 
    // of a new matrix, where one can be more efficient than the other. 
    fn out_of(&self, m: M) -> Self::Output;
}

/// A strategy trait for access to matrix elements.
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
