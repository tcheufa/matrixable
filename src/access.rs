//! Tools for matrix access and transformation.

use crate::strategies::*;

use crate::{ MatrixExt, MatrixMutExt };
use crate::req::MatrixExtFromIter;


/// A `MatrixExt` which provides immutable access to another matrix by following a certain access strategy.
/// 
/// This `struct` is created by the [`access`](crate::MatrixExt::access) method on `MatrixExt`. See its documentation for more.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Hash, Clone, Debug)]
pub struct Access<'a, M: MatrixExt, S: AccessStrategy<M>>{
    matrix: &'a M,
    pub strategy: S,
}

/// A `MatrixMutExt` which provides mutable access to another matrix by following a certain access strategy.
/// 
/// This `struct` is created by the [`access_mut`](crate::MatrixMutExt::access_mut) method on `MatrixMutExt`. See its documentation for more.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Hash, Debug)]
pub struct AccessMut<'a, M: MatrixExt, S: AccessStrategy<M>>{
    matrix: &'a mut M,
    pub strategy: S,
}

/// Used by [`AccessStrategySet`].
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Observer {
    pub rows: usize,
    pub cols: usize
}

// ### Self Implementation

impl<'a, M: MatrixExt, S: AccessStrategy<M>> Access<'a, M, S> {
    pub(crate) fn new(matrix: &'a M, strategy: S) -> Self {
        Self { matrix, strategy }
    }

    pub fn clone_into(&self) -> M
    where M: for<'b> MatrixExtFromIter<&'b M::Element> {
        MatrixExtFromIter::from_iter(self.iter(), self.num_cols())
    }
}

impl<'a, M: MatrixMutExt, S: AccessStrategy<M>> AccessMut<'a, M, S> {
    pub(crate) fn new(matrix: &'a mut M, strategy: S) -> Self {
        Self { matrix, strategy }
    }

    pub fn clone_into(&self) -> M
    where
        M: Clone,
        <M as MatrixExt>::Element: Clone
    {
        self.matrix.clone()
    }
}

impl Observer {
    #[inline]
    pub fn new(dimensions: (usize, usize)) -> Self {
        Self {
            rows: dimensions.0,
            cols: dimensions.1
        }
    }

    #[inline]
    pub fn update_dimensions(&mut self, s: &dyn AccessStrategy<Self>) {
        self.rows = s.nrows(self);
        self.cols = s.ncols(self);
    }
}



// ### MatrixExt Implementation

impl MatrixExt for Observer {
    type Element = ();

    #[inline(always)] fn num_rows(&self) -> usize { self.rows }
    #[inline(always)] fn num_cols(&self) -> usize { self.cols }

    #[inline(always)]
    fn get(&self, i: usize, j: usize) -> Option<&()> {
        if self.check(i, j) {
            return Some(&())
        } 
        None
    }
}

impl<'a, M: MatrixExt, S: AccessStrategy<M>> MatrixExt for Access<'a, M, S> {
    type Element = <M as MatrixExt>::Element;
    #[inline] fn num_rows(&self) -> usize { self.strategy.nrows(self.matrix) }
    #[inline] fn num_cols(&self) -> usize { self.strategy.ncols(self.matrix) }

    #[inline]
    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> {
        let (i, j) = self.strategy.access(self.matrix, row, column)?;
        self.matrix.get(i, j)
    }
}
impl<'a, M: MatrixMutExt, S: AccessStrategy<M>> MatrixExt for AccessMut<'a, M, S> {
    type Element = M::Element;

    #[inline] fn num_rows(&self) -> usize { self.strategy.nrows(self.matrix) }
    #[inline] fn num_cols(&self) -> usize { self.strategy.ncols(self.matrix) }

    #[inline]
    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = self.strategy.access(self.matrix, row, column)?;
        self.matrix.get(i, j) 
    }
}
impl<'a, M: MatrixMutExt, S: AccessStrategy<M>> MatrixMutExt for AccessMut<'a, M, S> {
    #[inline]
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element> { 
        let (i, j) = self.strategy.access(self.matrix, row, column)?;
        self.matrix.get_mut(i, j) 
    }
}

