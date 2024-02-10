//! Helpers for access and transformation of matrix.

//! TODO
//! - Create Strategy: ShiftFrontUnordered, ShiftBackUnordered, SubMatrix
//! - Discuss about the use of a closure instead of a fn pointer for access function type
//! - Create a Strategy version using a const Mapping i.e. `<const Mapping: M>`

use crate::traits::Matrix;
use crate::view::MatrixView;
use crate::traits::TransformStrategy;

/// This Strategy does nothing...
pub struct Identity;

/// A Strategy that performs matrix transposition.
pub struct Transposition;

/// A Strategy that performs a reverse clockwise rotation of the matrix.
pub struct RotationL;

/// A Strategy that performs a clockwise rotation of the matrix.
pub struct RotationR;

/// A Strategy that performs a horizontal flip of the matrix.
pub struct FlipH;

/// A Strategy that performs a vertical flip of the matrix.
pub struct FlipV;

/// A Strategy that performs a symmetry of elements by the center of the matrix.
pub struct CentralSymmetry;

/// A Strategy that performs a wrapped shift of elements from right to left.
pub struct ShiftFront<const N: usize>;

/// A Strategy that performs a wrapped shift of elements from left to right.
pub struct ShiftBack<const N: usize>;

/// Access to a matrix are conditioned by elements of another matrix.
pub struct AccessMap<M: Matrix, Map: Matrix = MatrixView<usize>> { 
    pub(crate) mapping: Map,
    pub(crate) target: M
} 



/// Matrix Wrapper which provides access to a matrix by following a certain access method.
pub struct Access<M: Matrix>{ 
    m: M,
    access: fn(&M, usize, usize) -> Option<(usize, usize)> 
}
impl<M: Matrix> Access<M> {
    pub(crate) fn new(m: M, access: fn(&M, usize, usize) -> Option<(usize, usize)>) -> Self {
        Self { m, access }
    }
    pub(crate) fn release(self) -> M { self.m }
}

impl<M: Matrix> Matrix for Access<M> {
    type Element = <M as Matrix>::Element;
    
    fn num_rows(&self) -> usize { self.m.num_cols() }

    fn num_cols(&self) -> usize { self.m.num_rows() }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = (self.access)(&self.m, row, column)?;
        self.m.get(i, j) 
    }

    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> {
        let (i, j) = (self.access)(&self.m, row, column)?;
        self.m.get_mut(i, j)
    }
}

// pub struct SubMatrix { start: usize, end: usize}
// pub struct CoordXY; Apparently same Transposition


impl<M: Matrix> TransformStrategy<M> for Identity {
//    fn copy_into(&self) -> M where &M::Element: Clone {}
    fn access(_m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((i, j))
    }
    fn transform(m: M) -> M { m }
}

impl<M: Matrix> TransformStrategy<M> for Transposition {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(_m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((j, i))
    }
    fn transform(mut m: M) -> M { 
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
            let limit = r * m.num_cols()  -  1;

            let mut hash = std::collections::HashSet::new();

            let mut dest: usize;
            let mut a;
            let mut b;
            
            for n in 1..limit {
                if hash.contains(&n) {
                    continue;
                }
                
                dest = (n * r) % limit;

                hash.insert(dest);
                
                a = m.indexes_from(n);
                b = m.indexes_from(dest);
                m.swap(a, b);
            }

            m.swap_dimensions();
            m
        }
    }
}

impl<M: Matrix> TransformStrategy<M> for RotationL {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(j)?.checked_sub(1)?,
            i
        ))
    }
    fn transform(m: M) -> M { 
        FlipV::transform(Transposition::transform(m))
    }
}


impl<M: Matrix> TransformStrategy<M> for RotationR {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            j,
            m.num_cols().checked_sub(i)?.checked_sub(1)?
        ))
    }
    fn transform(m: M) -> M {
        FlipH::transform(Transposition::transform(m))
    }
}


impl<M: Matrix> TransformStrategy<M> for FlipH {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            i,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }

    fn transform(mut m: M) -> M { 
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..rows {
            for j in 0..(cols / 2) {
                let a: *mut _ = m.get_mut(i, j).unwrap();
                let b: *mut _ = m.get_mut(i, cols - j - 1).unwrap();
                unsafe { std::mem::swap(&mut *a, &mut *b) };
            }
        }
        m
    }
}

impl<M: Matrix> TransformStrategy<M> for FlipV {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            j
        ))
    }

    fn transform(mut m: M) -> M { 
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

impl<M: Matrix> TransformStrategy<M> for CentralSymmetry {
//    fn copy_into(&self) -> M where &M::Element: Clone { }
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        Some((
            m.num_rows().checked_sub(i)?.checked_sub(1)?,
            m.num_cols().checked_sub(j)?.checked_sub(1)?
        ))
    }

    fn transform(mut m: M) -> M { 
        let cols = m.num_cols();
        let rows = m.num_rows();
        // no need to permute the middle row if number of rows is odd.
        for i in 0..(rows.div_ceil(2) - 1) {
            for j in 0..(cols.div_ceil(2) - 1) {
                m.swap((i, j), (rows - i - 1, cols - j - 1));
            }
        }
        m
    }
}

impl<M: Matrix, const N: usize> TransformStrategy<M> for ShiftBack<N> {
//    fn copy_into(&self) -> M where &M::Element: Clone {}
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut n = m.index_from((i, j));
        let len = m.size();
        
        n = (n + (N % len)) % len;
        
        Some(m.indexes_from(n))
    }
    
    fn transform(mut m: M) -> M { 
        let len = m.size();
        let shift = N % len;
        for i in 0..(len - shift) {
            let a = m.indexes_from(i);
            let b = m.indexes_from(i + shift);
            m.swap(a, b);
        }
        m
    }
}

impl<M: Matrix, const N: usize> TransformStrategy<M> for ShiftFront<N> {
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
    fn transform(mut m: M) -> M { 
        let len = m.size();
        let shift = N % len;
        
        let mut a;
        let mut b;
        let mid = N / 2;
        //let mid = ((len - 1) + (len - N))  /  2; 
//         println!("{mid}");
//         // Reorder the group to be shifted from lower to higher
//         for i in 0..N {
//             a = m.indexes_from(len - i - N);
//             b = m.indexes_from(mid - i );
//             m.swap(a, b);
//         }
        for i in (shift..len).rev() {
            a = m.indexes_from(i);
            b = m.indexes_from(i - shift);
            m.swap(a, b);
        }
        m
    }
}
