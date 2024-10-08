//! Structures implementing [`AccessStrategy`], [`InPlace`] and/or [`TransformStrategy`] traits.

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;


use crate::{ MatrixExt, MatrixMutExt };
use crate::access::Observer;

use ::core::ops::Deref;
use ::core::ops::{RangeBounds, RangeInclusive};

pub use crate::req::{ AccessStrategy, TransformStrategy, SwapsDimensions, InPlace };

/// This Strategy does nothing...
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, Identity };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ],
/// ];
///
/// // `m` is borrowed
/// let access = m.access(Identity);
/// 
/// assert!(m.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = Identity.out_of(m.clone()); 
///
/// assert_eq!(m, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct Identity;


/// Performs matrix transposition.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, Transpose };
///
/// let m = [
///    [ 0, 1 ],
///    [ 2, 3 ],
///    [ 4, 5 ]
/// ];
///
/// let expected = [
///    [ 0, 2, 4 ],
///    [ 1, 3, 5 ]
/// ];
///
/// // `m` is borrowed
/// let access = m.access(Transpose);
/// 
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = Transpose.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct Transpose;


/// Performs a clockwise rotation.
///
/// # Example
/// ```rust
/// use matrixable::{ MatrixExt, strategies::{ TransformStrategy, RotateR }};
///
/// let m = [
///    [ 0, 1 ],
///    [ 2, 3 ],
///    [ 4, 5 ]
/// ];
///
/// let expected =  [
///    [ 4, 2, 0 ],
///    [ 5, 3, 1 ]
/// ];
///
/// // `m` is borrowed.
/// let access = m.access(RotateR);
/// 
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = RotateR.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct RotateR;


/// Performs a counter-clockwise rotation.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, RotateL };
///
/// let m = [
///    [ 0, 1 ],
///    [ 2, 3 ],
///    [ 4, 5 ]
/// ];
///
/// // `m` is borrowed
/// let access = m.access(RotateL);
/// 
/// let expected = [
///    [ 1, 3, 5 ],
///    [ 0, 2, 4 ]
/// ];
///
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = RotateL.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct RotateL;


/// Performs a horizontal flip of a matrix.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, FlipH };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is borrowed.
/// let access = m.access(FlipH);
/// 
/// let expected = [
///    [ 2, 1, 0 ],
///    [ 5, 4, 3 ],
///    [ 8, 7, 6 ]
/// ];
///
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = FlipH.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct FlipH;


/// Performs a vertical flip of a matrix.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, FlipV };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is borrowed.
/// let access = m.access(FlipV);
/// 
/// let expected = [
///    [ 6, 7, 8 ],
///    [ 3, 4, 5 ],
///    [ 0, 1, 2 ]
/// ];
///
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = FlipV.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct FlipV;


/// Reverses a matrix by performing a symmetry of elements by the center of that matrix.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, Reverse };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is borrowed
/// let access = m.access(Reverse);
/// 
/// let expected = [
///    [ 8, 7, 6 ],
///    [ 5, 4, 3 ],
///    [ 2, 1, 0 ]
/// ];
///
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = Reverse.out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct Reverse;


/// Performs a circular shift of elements of a matrix from right to left.
///
/// # Field
/// `usize`: The number of front-shifts to be performed.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, ShiftFront };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is borrowed
/// let access = m.access(ShiftFront(2));
/// 
/// let expected = [
///    [ 7, 8, 0 ],
///    [ 1, 2, 3 ],
///    [ 4, 5, 6 ]
/// ];
/// 
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = ShiftFront(2).out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct ShiftFront(pub usize);


/// Performs a circular shift of elements from left to right.
///
/// # Field
/// `usize`: The number of back-shifts to be performed.
///
/// # Example
/// ```rust
///
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ TransformStrategy, ShiftBack };
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is borrowed
/// let access = m.access(ShiftBack(2));
/// 
/// let expected = [
///    [ 2, 3, 4 ],
///    [ 5, 6, 7 ],
///    [ 8, 0, 1 ]
/// ];
///
/// 
/// assert!(expected.rows().eq(access.rows()));
///
/// // `m` is consumed.
/// let t = ShiftBack(2).out_of(m); 
///
/// assert_eq!(expected, t);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct ShiftBack(pub usize);


/// Gives access to a portion of the matrix .
///
/// # Fields
/// 1. The list of indexes of rows we want to access.
/// 2. The list of indexes of columns we want to access.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::Submatrix;
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is always borrowed
/// let access = m.access(Submatrix(..=1, ..));
///
/// let expected = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ]
/// ];
/// assert!(expected.rows().eq(access.rows()));
///
/// // Out of bound indexes are simply brought back to the index bounds.
/// assert!(m.access(Submatrix(.., 0..=2)).rows().eq(m.access(Submatrix(0..3, 0..100)).rows()));
///
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct Submatrix<Rows: RangeBounds<usize>, Cols: RangeBounds<usize>>(pub Rows, pub Cols);


/// Gives a new shape to the matrix.
/// # Important
/// Remember that **The number of elements should remain the same.**
/// Otherwise a *panic* will occur when trying to access elements of the matrix using this strategy.
///
/// # Fields
/// 1. The new number of rows.
/// 2. The new number of columns.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::Reshape;
///
/// let m = [
///    [ 0, 1, 2 ],
///    [ 3, 4, 5 ],
///    [ 6, 7, 8 ]
/// ];
///
/// // `m` is always borrowed
/// let access = m.access(Reshape(9, 1));
///
/// let expected = [
///     [0],
///     [1],
///     [2],
///     [3],
///     [4],
///     [5],
///     [6],
///     [7],
///     [8],
/// ];
/// assert!(expected.rows().eq(access.rows()));
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord,  Clone, Copy, Debug)]
pub struct Reshape(pub usize, pub usize);


/// Accesses to a matrix (the subject) are defined by entries of another matrix (the map).
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::AccessMap;
///
/// let m = [
///     ['a', 'b', 'c'],
///     ['d', 'e', 'f'],
///     ['g', 'h', 'i']
/// ];
/// 
/// let mapping = [
///     [ 1, 0, 3, 0 ],
///     [ 4, 5, 6, 6 ],
///     [ 2, 8, 3, 8 ]
/// ];
/// 
/// // `m` is borrowed
/// let access = m.access(AccessMap(mapping));
/// 
/// let expected = [
///     ['b', 'a', 'd', 'a'],
///     ['e', 'f', 'g', 'g'],
///     ['c', 'i', 'd', 'i']
/// ];
/// 
/// assert!(expected.rows().eq(access.rows()));
/// ```
///
/// If an element of the mapping refers to no element in the subject, `None` will be returned,
/// when the [`MatrixExt::get`] method is called for that entry.
///
/// A consequence to this is that iterating over a matrix accessed by a mapping will stop as soon as
/// an incorrect entry is found inside the mapping. 
///
/// # Example 
/// ```should_panic
/// use matrixable::MatrixExt;
/// use matrixable::strategies::AccessMap;
///
/// let m = [[1, 3, 5], [100, 120, 140]];
///
/// let map = AccessMap([
///     [   1,    3,   5 ],
///     [ 100,  120, 140 ] 
/// ]);
///     
/// let access = m.access(map);
/// 
/// assert_eq!(Some(&3), access.get_nth(0)); // 0 => 1 => m(1) = 3
/// assert_eq!(Some(&100), access.get_nth(1)); // 1 => 3 => m(3) = 100
/// assert_eq!(Some(&140), access.get_nth(2)); // 2 => 5 => m(5) = 140
///
/// // Panics
/// assert_eq!(None, access.get_nth(3)); // 3 => 100 => m(100) out of bounds 
/// // Panics
/// assert_eq!(None, access.get_nth(4)); // 4 => 120 => m(120) out of bounds
/// // Panics
/// assert_eq!(None, access.get_nth(5)); // 5 => 140 => m(140) out of bounds
/// 
/// // Panics
/// assert_eq!(vec![&3, &100, &140], access.iter().collect::<Vec<_>>());
/// 
/// // However
/// assert_eq!(access.row(0).unwrap().len(), 3);
/// assert_eq!(access.row(1).unwrap().len(), 3);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AccessMap<Mapping: MatrixExt>(pub Mapping); 

/// This strategy access elements of this matrix following an ordered set of `AccessStrategy`s.
/// This set can only contain strategies implementing `AccessStrategy<Observer>`.
///
/// # Example
/// ```rust
/// use matrixable::MatrixExt;
/// use matrixable::strategies::{ ShiftFront, FlipH, Transpose, AccessStrategySet };
///
/// let m = [[0, 1], [2, 3]]; 
///
/// let strategy: AccessStrategySet = vec![
///     Box::new(ShiftFront(3)),
///     Box::new(Transpose),
///     Box::new(FlipH)
/// ];
///
/// let mut s = m.access(strategy);        
///
/// assert_eq!(Some(&3), s.get(0, 0));
/// assert_eq!(Some(&1), s.get(0, 1));
/// assert_eq!(Some(&0), s.get(1, 0));
/// assert_eq!(Some(&2), s.get(1, 1));
///
/// // remove `FlipH`
/// s.strategy.pop(); 
/// 
/// // add `Reverse`
/// use matrixable::strategies::Reverse;
///
/// s.strategy.push(Box::new(Reverse));
///
/// assert_eq!(Some(&0), s.get(0, 0));
/// assert_eq!(Some(&2), s.get(0, 1));
/// assert_eq!(Some(&3), s.get(1, 0));
/// assert_eq!(Some(&1), s.get(1, 1));
/// ```
/// <br/>
///
/// You can also access elements mutably, in a more advanced way than what a chaining of the [`MatrixMutExt::access_mut`] method can allow.
///
/// # Example
/// ```rust
/// use matrixable::MatrixMutExt;
/// use matrixable::strategies::{ AccessStrategySet, ShiftBack, Reverse };
///
/// let mut m = [[1, 2, 3, 4], [5, 6, 7, 8]];
///
/// let mut strategy: AccessStrategySet = vec![
///     Box::new(Reverse),
///     Box::new(ShiftBack(5))
/// ];
///
/// m.access_mut(&strategy)
///  .col_mut(0)
///  .unwrap()
///  .for_each(|x| *x = 11);      
///
/// assert_eq!([[1, 2, 11, 4], [5, 6, 11, 8]], m);
/// 
/// 
/// // add `AccessMap`
/// use matrixable::strategies::AccessMap;
///
/// let mapping = AccessMap([
///     [0, 0, 0, 3],
///     [0, 1, 3, 2],
///     [7, 7, 5, 0]
/// ]);
///
/// strategy.push(Box::new(mapping));
///
/// // Adds 1 to the element at position `mapping[i][j]` each time that
/// // position is found in the `mapping`. 
///
/// for row in m.access_mut(strategy).rows_mut() {
///     row.for_each(|x| *x += 1);
/// } 
///
/// // Rev-Shift: [[11, 2, 1, 8], [11, 6, 5, 4]]
/// // After mapped addition: [[16, 3, 2, 10], [11, 7, 5, 6]] 
/// // ShiftBack access removed: [[10, 11, 7, 5], [6, 16, 3, 2]] 
/// // Reverse access removed: [[2, 3, 16, 6], [5, 7, 11, 10]] 
///
/// assert_eq!([[2, 3, 16, 6], [5, 7, 11, 10]], m);
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub type AccessStrategySet = Vec<Box<dyn AccessStrategy<Observer>>>;

/// Sorts the matrix according to the result of a function.
/// If that argument function returns true, then its first parameter is considered
/// to come before the second parameter. The ordering can then be processed
/// based on that assumption.
///
/// # Note
/// The sorting algorithm used is **selection sort**. But this may change in a future version
/// (or not).
/// # Example
/// ```
/// use matrixable::MatrixMutExt;
/// use matrixable::strategies::SortBy;
///
/// let mut m = [
/// [4,  5,  6],
/// [9,  1, 20],
/// [4, 12, -1]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Hash, Clone, Copy, Debug)]
pub struct SortBy<T> (pub fn(&T, &T) -> bool);


// ### Self Impls

impl Transpose {
    /// In-place transposition optimized for square matrices.
    /// # Panics
    ///
    /// Panics if the matrix is not a square matrix.
    pub fn in_place_square<M: MatrixMutExt>(&self, m: &mut M) {
        if !m.is_square() {
            panic!("The matrix is not a square matrix.")
        }
        let dim = m.num_rows(); // or m.num_cols()
        for i in 0..dim {
            for j in 0..i {
                m.swap((i, j), (j, i));
            }
        }
    }
    
    /// Performs a regular in-place Transposition.
    pub fn in_place<M: SwapsDimensions + MatrixMutExt>(&self, m: &mut M) {
        // element [0] and element[size-1] does not need to be transposed
        // so we reduce the array into all the elements between indices 0 and size-1
        // that is `1..=size-2`
        let r = m.num_rows();
        let limit = m.size()  -  1;

        // Inspired by the algorithm from `GeeksForGeeks` website.
        let mut toreplace;
        let mut next ;
        let mut cycle_begin;

        #[cfg(not(feature = "std"))]
        let mut moved: Vec<usize> = vec![];
        
        #[cfg(feature = "std")]
        let mut moved: HashSet<usize> = std::collections::HashSet::new();

        let mut i = 1;
        while i < limit {
            cycle_begin = i;
            toreplace = i;
            loop {
                next = (i * r) % limit;
                m.swapn(toreplace, next);
                
                #[cfg(not(feature = "std"))]
                moved.push(i);
                
                #[cfg(feature = "std")]
                moved.insert(i);
                
                
                i = next;
                
                if i == cycle_begin {
                    break
                }
            }
            
            i = 1;
            while i < limit && moved.contains(&i) { i += 1 }
        } 
        m.swap_dimensions();
    }
}

impl Reverse {
    /// This method does nothing if an invalid range (such as `5..0`) is provided.
    /// 
    /// # Panics
    ///
    /// Panics if `start` or `end` are out of bounds.
    pub fn rev<M: MatrixMutExt>(&self, m: &mut M, range: ::core::ops::Range<usize>) {
        let mid = (range.start + range.end) / 2;
        for i in range.start..mid {
            m.swapn(i, range.end + range.start - i - 1);
        }
    }
    
    /// This method does nothing if an invalid range (such as `(1, 3)..(0, 0)`) is provided.
    ///
    /// # Panics
    ///
    /// Panics if `start` or `end` are out of bounds.
    pub fn rev2<M: MatrixMutExt>(&self, m: &mut M, range: ::core::ops::Range<(usize, usize)>) {
        let (start, end) = (m.index_from(range.start), m.index_from(range.end));
        self.rev(m, start..end);
    }
}

impl<Rows: RangeBounds<usize>, Cols: RangeBounds<usize>> Submatrix<Rows, Cols>
{
    fn get_range<R: RangeBounds<usize>>(len: usize, r: &R) 
        -> RangeInclusive<usize> {
        use ::core::ops::Bound;
        
        let start = match r.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(start) => *start + 1,
            Bound::Included(start) => *start,
        };
        
        // if start is out of bound return an empty range.
        if start >= len {
            return RangeInclusive::new(1, 0);
        }
        
        let end = match r.end_bound() {
            Bound::Excluded(&end) if end < len => end.saturating_sub(1),
            Bound::Included(&end) if end < len => end,
            // if end is out of bound bring it back to len - 1.
            _ => len.saturating_sub(1),
        };

        RangeInclusive::new(start, end)
    }
}




// ### AccessStrategy

impl <M: MatrixExt, S: AccessStrategy<M>> AccessStrategy<M> for &S {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        (*self).access(m, i, j)
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { (*self).nrows(m) }
    #[inline]
    fn ncols(&self, m: &M) -> usize { (*self).ncols(m) }
}

impl <M: MatrixExt> AccessStrategy<M> for Box<dyn AccessStrategy<M>> {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        self.deref().access(m, i, j)
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { self.deref().nrows(m) }
    #[inline]
    fn ncols(&self, m: &M) -> usize { self.deref().ncols(m) }
}

impl <M: MatrixExt> AccessStrategy<M> for &dyn AccessStrategy<M> {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        (*self).access(m, i, j)
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { (*self).nrows(m) }
    #[inline]
    fn ncols(&self, m: &M) -> usize { (*self).ncols(m) }
}


impl<M: MatrixExt> AccessStrategy<M> for Identity {
    #[inline]
    fn access(&self, _m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((i, j))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt> AccessStrategy<M> for Transpose {
    #[inline]
    fn access(&self, _m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((j, i))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_cols() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: MatrixExt> AccessStrategy<M> for RotateR {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(j)?.checked_sub(1)?,
            i
        ))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_cols() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: MatrixExt> AccessStrategy<M> for RotateL {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            j,
            m.num_cols().checked_sub(i)?.checked_sub(1)?
        ))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_cols() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: MatrixExt> AccessStrategy<M> for FlipH {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            i,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt> AccessStrategy<M> for FlipV {
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            j
        ))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt> AccessStrategy<M> for Reverse {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt> AccessStrategy<M> for ShiftBack {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut n = m.checked_index_from((i, j))?;
        let len = m.size();
        let shift = self.0 % len;
        
        if n >= len {
            return None
        }
        else if n >= len - shift {
            n -= len - shift;
        }
        else {
            n += shift;
        }
        
        Some(m.subscripts_from(n))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt> AccessStrategy<M> for ShiftFront {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut n = m.checked_index_from((i, j))?;
        let len = m.size();
        let shift = self.0 % len;

        if n >= len {
            return None
        }
        else if n >= shift {
            n -= shift;
        } 
        else {
            n += len - shift;
        }
        Some(m.subscripts_from(n))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { m.num_rows() }
    #[inline]
    fn ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: MatrixExt, Rows: RangeBounds<usize>, Cols: RangeBounds<usize>>
AccessStrategy<M> for Submatrix<Rows, Cols> {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let rows = Self::get_range(m.num_rows(), &self.0);
        let cols = Self::get_range(m.num_cols(), &self.1);
        
        if rows.is_empty() || cols.is_empty() {
            return None
        }
        
        if rows.contains(&i) && cols.contains(&j) {
            return Some((i, j))
        }
        else {
            return None
        }
    }
    
    fn nrows(&self, m: &M) -> usize { 
        let rows = Self::get_range(m.num_rows(), &self.0);
        if rows.is_empty() { 0 } 
        else {
            rows.end() - rows.start() + 1
        }
    }
    
    fn ncols(&self, m: &M) -> usize {
        let cols = Self::get_range(m.num_cols(), &self.1);
        if cols.is_empty() { 0 } 
        else {
            cols.end() - cols.start() + 1
        }
    }
}

impl<M: MatrixExt> AccessStrategy<M> for Reshape {
    /// # Panics
    /// Panics if dimensions given at `Reshape` initializtion does not fit the number of elements of the current matrix.
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        if m.size() != self.0 * self.1 {
            panic!("Reshape fails because dimensions provided {:?} does not fit the number of elements of the matrix ({})", self, m.size())
        }
        if i >= self.0 || j >= self.1 {
            None
        }
        else {
            Some(m.subscripts_from(i * self.1 + j))
        }
    }
    #[inline]
    fn nrows(&self, _m: &M) -> usize {
        self.0
    }

    #[inline]
    fn ncols(&self, _m: &M) -> usize {
        self.1
    }
}

impl<M: MatrixExt, Mapping: MatrixExt> AccessStrategy<M> for AccessMap<Mapping> 
    where for <'a> &'a <Mapping as MatrixExt>::Element: Into<&'a usize>
{
    /// # Panics
    ///
    /// Panics if an element of `Mapping` points to no element inside `m`.
    #[inline]
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let n = self.0.get(i, j)?.into();
        if !m.check_nth(*n) {
            panic!("Map index does not match with target matrix element.")
        }
        m.checked_subscripts_from(*n)
    }
    #[inline]
    fn nrows(&self, _m: &M) -> usize { self.0.num_rows() }
    #[inline]
    fn ncols(&self, _m: &M) -> usize { self.0.num_cols() }
}

impl <M: MatrixExt> AccessStrategy<M> for AccessStrategySet {
    #[inline]
    fn access(&self, m: &M, mut i: usize, mut j: usize) -> Option<(usize, usize)> {
        let observer = Observer::new(
            m.shape(),
        );
        for strategy in self.iter().rev() {
            (i, j) = strategy.deref().access(&observer, i, j)?;
        }
        Some((i, j))
    }
    #[inline]
    fn nrows(&self, m: &M) -> usize { 
        let mut observer = Observer::new(
            m.shape()
        );
        for strategy in self.iter().rev() {
            observer.update_dimensions(&strategy.deref());
        }
        observer.num_rows()
    }
    #[inline]
    fn ncols(&self, m: &M) -> usize {
        let mut observer = Observer::new(
            m.shape()
        );
        for strategy in self.iter().rev() {
            observer.update_dimensions(&strategy.deref());
        }
        observer.num_cols()
    }
}


// ### InPlace

impl<M, S> InPlace<M> for &S
where 
    M: MatrixMutExt, S: InPlace<M> 
{
    #[inline]
    fn in_place(&self, m: &mut M) {
        (*self).in_place(m)
    }
}

impl<M: MatrixMutExt> InPlace<M> for Identity {
    /// Does nothing internally.
    #[inline]
    fn in_place(&self, _m: &mut M) {}
}

impl<M: SwapsDimensions> InPlace<M> for Transpose {
    #[inline]
    fn in_place(&self, m: &mut M) {
        if m.is_square() {
            //much simpler
            self.in_place_square(m);
        }
        else {
            self.in_place(m);
        }
    }
}

impl<M: SwapsDimensions> InPlace<M> for RotateR 
where 
    Transpose: InPlace<M>,
    FlipH: InPlace<M>,
{
    #[inline]
    fn in_place(&self, m: &mut M) {
        Transpose.in_place(m);
        FlipH.in_place(m);
    }
}

impl<M: SwapsDimensions> InPlace<M> for RotateL
where 
    Transpose: InPlace<M>,
    FlipV: InPlace<M>,
{
    #[inline]
    fn in_place(&self, m: &mut M) {
        Transpose.in_place(m);
        FlipV.in_place(m);
    }
}

impl<M: MatrixMutExt> InPlace<M> for FlipH { 
    fn in_place(&self, m: &mut M) {
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..rows {
            for j in 0..(cols / 2) {
                m.swap((i, j), (i, cols - j - 1));
            }
        }
    }
}

impl<M: MatrixMutExt> InPlace<M> for FlipV {
    fn in_place(&self, m: &mut M) {
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..(rows / 2) {
            for j in 0..cols {
                m.swap((i, j), (rows - i - 1, j));
            }
        }
    }
}

impl<M: MatrixMutExt> InPlace<M> for Reverse {
    #[inline]
    fn in_place(&self, m: &mut M) {
        Reverse.rev(m, 0..m.size());
    }
}

impl<M: MatrixMutExt> InPlace<M> for ShiftBack {
    // Does nothing if shift equals 0
    fn in_place(&self, m: &mut M) {
        let len = m.size();
        let shift = self.0 % len;
        
        if shift == 0 { 
            return;
        }
        
        {
            let reverse = Reverse;
            reverse.rev(m, 0..len);
            reverse.rev(m, len-shift..len);
            reverse.rev(m, 0..len-shift);
        }
    }
}

impl<M: MatrixMutExt> InPlace<M> for ShiftFront {
    fn in_place(&self, m: &mut M) {
        let len = m.size();
        let shift = self.0 % len;
        
        if shift == 0 { 
            return;
        }
        
        {
            let reverse = Reverse;
            reverse.rev(m, 0..len);
            reverse.rev(m, 0..shift);
            reverse.rev(m, shift..len);
        }
    }
}

impl<M: MatrixMutExt> InPlace<M> for SortBy<M::Element> {
    fn in_place(&self, m: &mut M) {
        let mut im;
        let mut min;
        let mut cmp;

        for i in 0..(m.size() - 1) {
            im = i;
            min = m.get_nth(i).unwrap();
            for j in (i+1)..m.size() {
                cmp = m.get_nth(j).unwrap();
                if !(self.0)(min, cmp) {
                    im = j;
                    min = cmp;
                }
            }
            m.swapn(im, i);
        }
    }
}

// ### TransformStrategy

impl<M, S> TransformStrategy<M> for &S 
where 
    M: MatrixExt,
    S: TransformStrategy<M>
{
    type Output = <S as TransformStrategy<M>>::Output;

    #[inline]
    fn out_of(&self, m: M) -> Self::Output { (*self).out_of(m) }
}

impl<M: MatrixExt> TransformStrategy<M> for Identity {
    type Output = M;

    #[inline]
    fn out_of(&self, m: M) -> Self::Output { m }
}

impl<M: SwapsDimensions + MatrixMutExt > TransformStrategy<M> for Transpose {
    type Output = M;
    
    fn out_of(&self, mut m: M) -> Self::Output {
        if m.is_square() {
            //much simpler
            self.in_place_square(&mut m);
            m
        }
        else {
            self.in_place(&mut m);
            m
        }
    }
}

impl<M: MatrixExt> TransformStrategy<M> for RotateR 
where 
    Transpose: TransformStrategy<M>,
    <Transpose as TransformStrategy<M>>::Output: MatrixExt,
    FlipH: TransformStrategy<<Transpose as TransformStrategy<M>>::Output>
{
    type Output = <FlipH as TransformStrategy<<Transpose as TransformStrategy<M>>::Output>>::Output;

    #[inline]
    fn out_of(&self, m: M) -> Self::Output {
        FlipH.out_of(Transpose.out_of(m))
    }
}

impl<M: MatrixExt> TransformStrategy<M> for RotateL
where
    Transpose: TransformStrategy<M>,
    <Transpose as TransformStrategy<M>>::Output: MatrixExt,
    FlipV: TransformStrategy<<Transpose as TransformStrategy<M>>::Output>
{
    type Output = <FlipV as TransformStrategy<<Transpose as TransformStrategy<M>>::Output>>::Output;

    #[inline]
    fn out_of(&self, m: M) -> Self::Output {
        FlipV.out_of(Transpose.out_of(m))
    }
}

impl<M: MatrixMutExt> TransformStrategy<M> for FlipH {
    type Output = M;
    
    fn out_of(&self, mut m: M) -> Self::Output {
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..rows {
            for j in 0..(cols / 2) {
                m.swap((i, j), (i, cols - j - 1));
            }
        }
        m
    }
}

impl<M: MatrixMutExt> TransformStrategy<M> for FlipV {
    type Output = M;
    
    fn out_of(&self, mut m: M) -> Self::Output {
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..(rows / 2) {
            for j in 0..cols {
                m.swap((i, j), (rows - i - 1, j));
            }
        }
        m
    }
}

impl<M: MatrixMutExt> TransformStrategy<M> for Reverse {
    type Output = M;

    #[inline]
    fn out_of(&self, mut m: M) -> Self::Output {
        let len = m.size();
        Reverse.rev(&mut m, 0..len);
        m
    }
}

impl<M: MatrixMutExt> TransformStrategy<M> for ShiftBack {
    type Output = M;
    
    fn out_of(&self, mut m: M) -> Self::Output {
        let len = m.size();
        let shift = self.0 % len;
        
        if shift == 0 { 
            return m 
        }
        
        {
            let reverse = Reverse;
            reverse.rev(&mut m, 0..len);
            reverse.rev(&mut m, len-shift..len);
            reverse.rev(&mut m, 0..len-shift);
        }
        m
    }
}

impl<M: MatrixMutExt> TransformStrategy<M> for ShiftFront {
    type Output = M;
    
    fn out_of(&self, mut m: M) -> Self::Output {
        let len = m.size();
        let shift = self.0 % len;
        
        if shift == 0 { 
            return m 
        }
        
        {
            let reverse = Reverse;
            reverse.rev(&mut m, 0..len);
            reverse.rev(&mut m, 0..shift);
            reverse.rev(&mut m, shift..len);
        }
        m
    }
}
