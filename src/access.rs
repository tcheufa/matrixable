//! Helpers for access and transformation of matrix.
//!
//! TODO
//! - Create Strategy: ShiftFrontUnordered, ShiftBackUnordered, SubMatrix
//! - Discuss about the use of a closure instead of a fn pointer for access function type
//! - Create a Strategy version using a const Mapping i.e. `<const Mapping: M>`

use crate::traits::{Matrix, MatrixMut, SwapDimensions};
use crate::view::MatrixView;
use crate::traits::TransformStrategy;

/// This Strategy does nothing...
pub struct Identity;

/// Performs matrix transposition.
pub struct Transpose;

/// Performs a counter-clockwise rotation.
pub struct RotateL;

/// Performs a clockwise rotation.
pub struct RotateR;

/// that performs a horizontal flip of a matrix.
pub struct FlipH;

/// A Strategy that performs a vertical flip of a matrix.
pub struct FlipV;

/// Reverses a matrix by performing a symmetry of elements by the center of that matrix.
pub struct Reverse;

/// A Strategy that performs a wrapped shift of elements of a matrix from right to left.
pub struct ShiftFront<const N: usize>;

/// A Strategy that performs a wrapped shift of elements from left to right.
pub struct ShiftBack<const N: usize>;

pub struct SubMatrix<const START:(usize, usize), const END: Option<(usize, usize)>>;
// /// Access to a matrix are conditioned by elements of another matrix.
// pub struct AccessMap<M: Matrix, Map: Matrix = MatrixView<usize>> { 
//     pub(crate) mapping: Map,
//     pub(crate) target: M
// } 


// pub struct SubMatrix { start: usize, end: usize}
// pub struct CoordXY; Apparently same Transposition


impl<M: Matrix> TransformStrategy<M> for Identity {
//    fn copy_into(&self) -> M where &M::Element: Clone {}
    fn access(_m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((i, j))
    }
    fn out_of(m: M) -> M { m }
}

impl<M: MatrixMut + SwapDimensions> TransformStrategy<M> for Transpose {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(_m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((j, i))
    }
    
    fn out_of(mut m: M) -> M { 
        if m.is_square() {
            //much more simpler
            let dim = m.num_rows(); // or m.num_cols()
            for i in 0..dim {
                for j in 0..i {
                    m.swap((i, j), (j, i));
                }
            }
            m
        }
        else {
            // element [0] and element[size-1] does not need to be transposed
            // so we reduce the array into all the elements between indices 0 and size-1
            // that is `1..=size-2`
            let r = m.num_rows();
            let limit = m.size()  -  1;

            let mut toreplace;
            let mut next ;
            let mut cycle_begin;
        
            let mut moved: std::collections::HashSet<usize> = std::collections::HashSet::new();
            
//             moved.insert(0);
//             moved.insert(limit);

            let mut i = 1;
            while i < limit {
                cycle_begin = i;
                toreplace = i;
                loop {
                    next = (i * r) % limit;
                    m.swapn(toreplace, next);
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
            m
        }
    }
}

impl<M: MatrixMut + SwapDimensions> TransformStrategy<M> for RotateL {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(j)?.checked_sub(1)?,
            i
        ))
    }
    fn out_of(m: M) -> M { 
        FlipV::out_of(Transpose::out_of(m))
    }
}


impl<M: MatrixMut + SwapDimensions> TransformStrategy<M> for RotateR {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            j,
            m.num_cols().checked_sub(i)?.checked_sub(1)?
        ))
    }
    fn out_of(m: M) -> M {
        FlipH::out_of(Transpose::out_of(m))
    }
}


impl<M: MatrixMut> TransformStrategy<M> for FlipH {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            i,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }

    fn out_of(mut m: M) -> M { 
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

impl<M: MatrixMut> TransformStrategy<M> for FlipV {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            j
        ))
    }

    fn out_of(mut m: M) -> M { 
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

impl<M: MatrixMut> TransformStrategy<M> for Reverse {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }

    fn out_of(mut m: M) -> M { 
        let len = m.size();
        for i in 0..(len/2) {
            m.swapn(i, len - i - 1);
        }
        m
    }
}


impl<M: MatrixMut, const N: usize> TransformStrategy<M> for ShiftBack<N> {
//    fn copy_into(&self) -> M where &M::Element: Clone {}
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut n = m.index_from((i, j));
        let len = m.size();
        
        n = (n + (N % len)) % len;
        
        Some(m.indexes_from(n))
    }
    
    fn out_of(mut m: M) -> M { 
        let len = m.size();
        let shift = N % len;
        for i in 0..(len - shift) {
            m.swapn(i, i + shift);
        }
        m
    }
}

impl<M: MatrixMut, const N: usize> TransformStrategy<M> for ShiftFront<N> {
//    fn copy_into(&self) -> M where &M::Element: Clone {}
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut n = m.index_from((i, j));
        let len = m.size();
        if n < N {
            n = len - (N % len) + n;
        }
        else {
            n = n - N;
        }
        Some(m.indexes_from(n))
    }
    
    fn out_of(m: M) -> M {
        let len = m.size();
        let shift = N % len;
        
        if shift == 0 { return m }
        
        let mut rev = Reverse::out_of(m);
        
        for i in 0..(shift / 2) {
            rev.swapn(i, shift - i - 1);
        }
        
        let len = len + shift;
        for i in shift..(len / 2) {
            rev.swapn(i, len - i - 1);
        }
        rev
    }
}

impl<const START: (usize, usize), const END: Option<(usize, usize)>, M: Matrix>
    TransformStrategy<M> for SubMatrix<START, END> {
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let idx = START + (i, j);
        if idx > END? {
            None
        }
        else {
            Some(idx)
        }
    }
    
    fn out_of(m: M) -> M { m }
}

/// Matrix Wrapper which provides immutable access to a matrix by following a certain access method.
#[derive(Clone)]
pub struct Access<'a, M: Matrix>{
    m: &'a M,
    access: fn(&M, usize, usize) -> Option<(usize, usize)> 
}

/// Matrix Wrapper which provides mutable matrix access to a matrix by following a certain access method.
pub struct AccessMut<'a, M: Matrix>{
    m: &'a mut M,
    access: fn(&M, usize, usize) -> Option<(usize, usize)> 
}


impl<'a, M: Matrix> Access<'a, M> {
    pub(crate) fn new(m: &'a M, access: fn(&M, usize, usize) -> Option<(usize, usize)>) -> Self {
        Self { m, access }
    }
}
impl<'a, M: Matrix> AccessMut<'a, M> {
    pub(crate) fn new(m: &'a mut M, access: fn(&M, usize, usize) -> Option<(usize, usize)>) -> Self {
        Self { m, access }
    }
}

impl<'a, M: Matrix> Matrix for Access<'a, M> {
    type Element = <M as Matrix>::Element;
    
    fn num_rows(&self) -> usize { self.m.num_cols() }

    fn num_cols(&self) -> usize { self.m.num_rows() }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = (self.access)(&self.m, row, column)?;
        self.m.get(i, j) 
    }
}

impl<'a, M: MatrixMut> Matrix for AccessMut<'a, M> {
    type Element = M::Element;
    
    fn num_rows(&self) -> usize { self.m.num_cols() }

    fn num_cols(&self) -> usize { self.m.num_rows() }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = (self.access)(&self.m, row, column)?;
        self.m.get(i, j) 
    }
}
impl<'a, M: MatrixMut> MatrixMut for AccessMut<'a, M> {
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> { 
        let (i, j) = (self.access)(&self.m, row, column)?;
        self.m.get_mut(i, j) 
    }
}
