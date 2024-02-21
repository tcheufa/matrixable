//! Helpers for matrix access and transformation.

use crate::traits::{ Matrix, MatrixMut, SwapDimensions };
use crate::traits::{ AccessStrategy, MatrixFromIter, TransformStrategy };

/// This Strategy does nothing...
#[derive(Clone, Copy)]
pub struct Identity;


/// Performs matrix transposition.
#[derive(Clone, Copy)]
pub struct Transpose;


/// Performs a clockwise rotation.
#[derive(Clone, Copy)]
pub struct RotateR;


/// Performs a counter-clockwise rotation.
#[derive(Clone, Copy)]
pub struct RotateL;


/// Performs a horizontal flip of a matrix.
#[derive(Clone, Copy)]
pub struct FlipH;


/// Performs a vertical flip of a matrix.
#[derive(Clone, Copy)]
pub struct FlipV;


/// Reverses a matrix by performing a symmetry of elements by the center of that matrix.
#[derive(Clone, Copy)]
pub struct Reverse;


/// Performs a wrapped shift of elements of a matrix from right to left.
///
/// # Field
/// `usize`: The number of front-shifts to be performed.
#[derive(Clone, Copy)]
pub struct ShiftFront(pub usize);


/// Performs a wrapped shift of elements from left to right.
///
/// # Field
/// `usize`: The number of back-shifts to be performed.
#[derive(Clone, Copy)]
pub struct ShiftBack(pub usize);


/// Gives a matrix by deleting a row and a column from the original matrix.
/// 
/// # Fields
/// - `usize`(1): The first field is the index of the row to delete
/// - `usize`(2): The second field is the index of the column to delete.
#[derive(Clone)]
pub struct SubMatrix(usize, usize);

/// Access a matrix are defined by entries of another matrix.
#[derive(Clone)]
pub struct AccessMap<Mapping: Matrix>(pub Mapping); 

impl Reverse {
    /// Panics if those `start` or `end`S are out of bounds.
    pub fn rev<M: MatrixMut>(&self, m: &mut M, range: std::ops::Range<usize>) {
        let mid = (range.start + range.end) / 2;
        for i in range.start..mid {
            m.swapn(i, range.end + range.start - i - 1);
        }
    }
    
    /// Panics if those `start` or `end`S are out of bounds.
    pub fn rev2<M: MatrixMut>(&self, m: &mut M, range: std::ops::Range<(usize, usize)>) {
        
        let (start, end) = (m.index_from(range.start), m.index_from(range.end));
        self.rev(m, start..end);
    }
}

impl SubMatrix {
    /// # Panic
    ///  Unlike other AccessStrategy implmentations this method will panic in the case where the row and the column to delete does not exsit 
    pub fn of<M: Matrix>(m: &M, delrow: usize, delcol: usize) -> Self {
        if !m.check(delrow, delcol) {
            panic!("Row index or column index does not exist");
        }
        SubMatrix(delrow, delcol)
    }
}


/// A Strategy for matrix elements' access.
impl<M: Matrix> AccessStrategy<M> for Identity {
    fn access(&self, _m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((i, j))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for Transpose {
    fn access(&self, _m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((j, i))
    }
    
    fn new_nrows(&self, m: &M) -> usize { m.num_cols() }
    fn new_ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: Matrix> AccessStrategy<M> for RotateR {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(j)?.checked_sub(1)?,
            i
        ))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_cols() }
    fn new_ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: Matrix> AccessStrategy<M> for RotateL {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            j,
            m.num_cols().checked_sub(i)?.checked_sub(1)?
        ))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_cols() }
    fn new_ncols(&self, m: &M) -> usize { m.num_rows() }
}

impl<M: Matrix> AccessStrategy<M> for FlipH {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            i,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for FlipV {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            j
        ))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for Reverse {
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for ShiftBack {
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
        
        Some(m.indexes_from(n))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for ShiftFront {
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
        Some(m.indexes_from(n))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() }
}

impl<M: Matrix> AccessStrategy<M> for SubMatrix {
    fn access(&self, _m: &M, mut i: usize, mut j: usize) -> Option<(usize, usize)> {
        if i >= self.0 { i += 1 }
        if j >= self.1 { j += 1 }
        Some((i, j))
    }
    fn new_nrows(&self, m: &M) -> usize { m.num_rows() - 1 }
    fn new_ncols(&self, m: &M) -> usize { m.num_cols() - 1 }
}

impl<M: Matrix, Mapping: Matrix> AccessStrategy<M> for AccessMap<Mapping>
where for<'a> &'a Mapping::Element: Into<&'a usize>
{
    fn access(&self, m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let idx: &usize = self.0.get(i, j)?.into();
        Some(m.indexes_from(*idx))
    }
    fn new_nrows(&self, _m: &M) -> usize { self.0.num_rows() }
    fn new_ncols(&self, _m: &M) -> usize { self.0.num_cols() }
}


/// Matrix Wrapper which provides immutable access to a matrix by following a certain access method.
#[derive(Clone)]
pub struct Access<'a, M: Matrix, S: AccessStrategy<M>>{
    matrix: &'a M,
    strategy: S,
}

/// Matrix Wrapper which provides mutable matrix access to a matrix by following a certain access method.
pub struct AccessMut<'a, M: Matrix, S: AccessStrategy<M>>{
    matrix: &'a mut M,
    strategy: S,
}


impl<'a, M: Matrix, S: AccessStrategy<M>> Access<'a, M, S> {
    pub(crate) fn new(matrix: &'a M, strategy: S) -> Self {
        Self { matrix, strategy }
    }
    
    pub fn clone_into(&self) -> M 
    where M: for<'b> MatrixFromIter<&'b M::Element> {
        MatrixFromIter::from_iter(self.iter(), self.num_rows(), self.num_cols())
    }
}
impl<'a, M: MatrixMut, S: AccessStrategy<M>> AccessMut<'a, M, S> {
    pub(crate) fn new(matrix: &'a mut M, strategy: S) -> Self {
        Self { matrix, strategy }
    }
        
    pub fn clone_into(&self, m: &M) -> M 
    where 
        M: Clone,
        <M as Matrix>::Element: Clone
    {
        let mut m2 = m.clone();
        self.matrix
                .iter()
                .zip(m2.iter_mut())
                .for_each(|(&ref x, x2)| *x2 = x.clone());
        m2
    }
}

impl<'a, M: Matrix, S: AccessStrategy<M>> Matrix for Access<'a, M, S> {
    type Element = <M as Matrix>::Element;
    
    fn num_rows(&self) -> usize { self.strategy.new_nrows(&self.matrix) }

    fn num_cols(&self) -> usize { self.strategy.new_ncols(&self.matrix) }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = self.strategy.access(&self.matrix, row, column)?;
        self.matrix.get(i, j)
    }
}

impl<'a, M: MatrixMut, S: AccessStrategy<M>> Matrix for AccessMut<'a, M, S> {
    type Element = M::Element;
    
    fn num_rows(&self) -> usize { self.strategy.new_nrows(&self.matrix) }

    fn num_cols(&self) -> usize { self.strategy.new_ncols(&self.matrix) }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = self.strategy.access(&self.matrix, row, column)?;
        self.matrix.get(i, j) 
    }
}
impl<'a, M: MatrixMut, S: AccessStrategy<M>> MatrixMut for AccessMut<'a, M, S> {
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> { 
        let (i, j) = self.strategy.access(&self.matrix, row, column)?;
        self.matrix.get_mut(i, j) 
    }
}


impl<M: Matrix> TransformStrategy<M> for Identity {
    fn out_of(&self, m: M) -> M {
        m
    }
}


impl<M: MatrixMut + SwapDimensions> TransformStrategy<M> for Transpose {
    fn out_of(&self, mut m: M) -> M {
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
    fn out_of(&self, m: M) -> M {
        FlipV.out_of(Transpose.out_of(m))
    }
}


impl<M: MatrixMut + SwapDimensions> TransformStrategy<M> for RotateR {
    fn out_of(&self, m: M) -> M {
        FlipH.out_of(Transpose.out_of(m))
    }
}


impl<M: MatrixMut> TransformStrategy<M> for FlipH {
    fn out_of(&self, mut m: M) -> M {
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
    fn out_of(&self, mut m: M) -> M {
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
    fn out_of(&self, mut m: M) -> M {
        let len = m.size();
        Reverse.rev(&mut m, 0..len);
        m
    }
}


impl<M: MatrixMut> TransformStrategy<M> for ShiftBack {
    fn out_of(&self, mut m: M) -> M {
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

impl<M: MatrixMut> TransformStrategy<M> for ShiftFront {
    fn out_of(&self, mut m: M) -> M {
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
