//! Tools for matrix access and transformation.

use crate::strategies::*;

use crate::{ Matrix, MatrixMut };
use crate::req::MatrixFromIter;


/// A `Matrix` which provides immutable access to another matrix by following a certain access strategy.
/// 
/// This `struct` is created by the [`access`](crate::Matrix::access) method on `Matrix`. See its documentation for more.
#[derive(Clone, Debug)]
pub struct Access<'a, M: Matrix, S: AccessStrategy<M>>{
    matrix: &'a M,
    pub strategy: S,
}

/// A `MatrixMut`' which provides mutable access to another matrix by following a certain access strategy.
/// 
/// This `struct` is created by the [`access_mut`](crate::MatrixMut::access_mut) method on `MatrixMut`. See its documentation for more.
#[derive(Debug)]
pub struct AccessMut<'a, M: Matrix, S: AccessStrategy<M>>{
    matrix: &'a mut M,
    strategy: S,
}

/// Used by [`AccessStrategySet`].
#[derive(Clone, Copy, Debug)]
pub struct Observer {
    rows: usize,
    cols: usize
}

// ### Self Implementation

impl<'a, M: Matrix, S: AccessStrategy<M>> Access<'a, M, S> {
    pub(crate) fn new(matrix: &'a M, strategy: S) -> Self {
        Self { matrix, strategy }
    }
    
    pub fn clone_into2(&self) -> M 
    where M: for<'b> MatrixFromIter<&'b M::Element> {
        MatrixFromIter::from_iter(self.iter(), self.num_cols())
    }
}
impl<'a, M: MatrixMut, S: AccessStrategy<M>> Access<'a, M, S> {    
    pub fn clone_into(&self) -> M 
    where 
        M: Clone,
        <M as Matrix>::Element: Clone
    {
        self.matrix.duplicate()
    }
}

impl<'a, M: MatrixMut, S: AccessStrategy<M>> AccessMut<'a, M, S> {
    pub(crate) fn new(matrix: &'a mut M, strategy: S) -> Self {
        Self { matrix, strategy }
    }
        
    pub fn clone_into(&self) -> M 
    where 
        M: Clone,
        <M as Matrix>::Element: Clone
    {
        self.matrix.duplicate()
    }
}

impl Observer {
    pub fn new(dimensions: (usize, usize)) -> Self {
        Self {
            rows: dimensions.0,
            cols: dimensions.1
        }
    }
    
    pub fn update_dimensions(&mut self, s: &dyn AccessStrategy<Self>) {
        self.rows = s.nrows(&self);
        self.cols = s.ncols(&self);
    }
}



// ### Matrix Implementation

impl Matrix for Observer {
    type Element = ();
    
    fn num_rows(&self) -> usize { self.rows }
    fn num_cols(&self) -> usize { self.cols }
    fn get(&self, i: usize, j: usize) -> Option<&()> { 
        if self.check(i, j) {
            return Some(&())
        } 
        None
    }
}

impl<'a, M: Matrix, S: AccessStrategy<M>> Matrix for Access<'a, M, S> {
    type Element = <M as Matrix>::Element;
    
    fn num_rows(&self) -> usize { self.strategy.nrows(&self.matrix) }
    fn num_cols(&self) -> usize { self.strategy.ncols(&self.matrix) }

    fn get(&self, row: usize, column: usize) -> Option<&Self::Element> { 
        let (i, j) = self.strategy.access(&self.matrix, row, column)?;
        self.matrix.get(i, j)
    }
}
impl<'a, M: MatrixMut, S: AccessStrategy<M>> Matrix for AccessMut<'a, M, S> {
    type Element = M::Element;
    
    fn num_rows(&self) -> usize { self.strategy.nrows(&self.matrix) }
    fn num_cols(&self) -> usize { self.strategy.ncols(&self.matrix) }

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

