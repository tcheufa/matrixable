#![no_std]

//! This library contains traits that provides matrix behavior to its implementors and also helper structs for ease of use.
//!
//! A matrix implementing [`MatrixExt`] is by default in *[`Row Major Order`]*, but you can still change it using transpose access.
//!
//! # Features
//! * **impls** (default): Enables implementation of `MatrixExt` and `MatrixMutExt` for the standard 2D array `[[T; N]; M]`.
//!
//! [`Row Major Order`]: https://en.m.wikipedia.org/wiki/Row-_and_column-major_order

pub mod access;
pub mod iterators;  
pub mod req;
pub mod strategies;

pub mod prelude {
    pub use crate::{MatrixExt, MatrixMutExt};
    pub use crate::strategies::*;
    pub use crate::req::*;
    pub use crate::access::Observer;
    pub use crate::{print_rows_debug, print_columns_debug, print_diagonals_debug};
}

#[cfg(feature = "impls")]
mod impls;

extern crate alloc;
extern crate std;

use alloc::vec::Vec;

pub fn print_rows_debug<M: MatrixExt> (p: &M) where <M as MatrixExt>::Element: ::core::fmt::Debug {
    use std::println;
    println!("Rows");
    p.rows().enumerate().for_each(|(i, row)| println!("{i}: {:?}", row.collect::<Vec<_>>()))
}

pub fn print_columns_debug<M: MatrixExt> (p: &M) where <M as MatrixExt>::Element: ::core::fmt::Debug {
    use ::std::println;
    println!("Columns");
    p.cols().enumerate().for_each(|(i, col)| println!("{i}: {:?}", col.collect::<Vec<_>>()))
}

pub fn print_diagonals_debug<M: MatrixExt> (p: &M) where <M as MatrixExt>::Element: ::core::fmt::Debug {
    use std::println;
    println!("Diagonals");
    p.diags().enumerate().for_each(|(i, diag)| println!("{i}: {:?}", diag.collect::<Vec<_>>()))
}


use crate::iterators::*;
use crate::access::{Access, AccessMut};
use req::*;

/// This trait provides methods and tools for accessing data in matrix-like structures.
///
/// This trait allows only immutable access to elements of a matrix.
/// For a mutable implementation see [`MatrixMutExt`]
pub trait MatrixExt
{
    /// The type of the elements of the matrix.
    type Element;
    
    // Required methods
    
    /// Gets the number of rows of the matrix.
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let a = [[1, 2, 3]];
    /// assert_eq!(a.num_rows(), 1);
    /// 
    /// let empty: [[(); 0]; 0]  = []; 
    /// assert_eq!(empty.num_rows(), 0);
    /// 
    /// let empty1: [[(); 0]; 1]  = [[]]; 
    /// assert_eq!(empty1.num_rows(), 0);
    /// 
    /// let empty2: [[(); 0]; 2]  = [[], []]; 
    /// assert_eq!(empty2.num_rows(), 0);
    /// ```
    fn num_rows(&self) -> usize;

    /// Gets the number of columns of the matrix.
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let a = [[1, 2, 3]];
    /// assert_eq!(a.num_cols(), 3);
    /// 
    /// let empty: [[(); 0]; 0]  = []; 
    /// assert_eq!(empty.num_cols(), 0);
    /// 
    /// let empty1: [[(); 0]; 1]  = [[]]; 
    /// assert_eq!(empty1.num_cols(), 0);
    /// 
    /// let empty2: [[(); 0]; 2]  = [[], []]; 
    /// assert_eq!(empty2.num_cols(), 0);
    /// ```
    fn num_cols(&self) -> usize;

    /// Returns a reference to an element inside the matrix, at the intersection of the `i`-th row and the `j`-th column.
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let v = [[10, 40, 30]];
    ///
    /// assert_eq!(Some(&40), v.get(0, 1));
    /// assert_eq!(None, v.get(0, 3));
    /// ```
    fn get(&self, row: usize, column: usize) -> Option<&Self::Element>;

    
    // Provided methods.
    
    /// Returns a reference to an element, without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get(row_index, column_index).unwrap_unchecked()`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let x = &[[1, 2, 4]];
    ///
    /// unsafe {
    ///     assert_eq!(x.get_unchecked(0, 1), &2);
    /// }
    /// ```
    /// [`get`]: crate::MatrixExt::get
    unsafe fn get_unchecked(&self, row: usize, column: usize) -> &Self::Element {
        self.get(row, column).unwrap_unchecked()
    }
    
    /// Gets a reference to an element inside a matrix, given its order of disposition in *Row Major Order*.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let v = [[10, 40, 30]];
    ///
    /// assert_eq!(Some(&40), v.get_nth(1));
    /// assert_eq!(None, v.get_nth(3));
    /// ```
    #[inline]
    fn get_nth(&self, n: usize) -> Option<&Self::Element> {
        let (i, j) = self.subscripts_from(n);
        self.get(i, j)
    }
        
    #[inline]
    /// Returns the first element of the  matrix, or `None` if it is empty.
    fn first(&self) -> Option<&Self::Element> {
        self.get(0, 0)
    }

    #[inline]
    /// Returns the last element of the  matrix, or `None` if it is empty.
    fn last(&self) -> Option<&Self::Element> {
        match self.shape() {
            (0, _) | (_, 0) => None,
            (r, c) => self.get(r - 1, c - 1)
        }
    }

    /// Returns a reference to an element given its linear order, without doing bound checking.
    ///
    /// For a safe alternative see [`get_nth`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_nth(index).unwrap_unchecked()`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let x = &[[1, 2, 4]];
    ///
    /// unsafe {
    ///     assert_eq!(x.get_nth_unchecked(1), &2);
    /// }
    /// ```
    /// [`get_nth`]: crate::MatrixExt::get_nth
    unsafe fn get_nth_unchecked(&self, n: usize) -> &Self::Element {
        let (i, j) = self.subscripts_from(n);
        self.get_unchecked(i, j)
    }

    /// Returns the number of elements of the matrix
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert_eq!(5, [[1, 2, 3, 4, 5]].size());
    /// assert_eq!(6, [[1, 2], [3, 4], [5, 6]].size());
    /// ```
    #[inline]
    fn size(&self) -> usize { self.num_rows() * self.num_cols() }

    /// Returns the dimensions of the matrix
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [[1, 1, 1], [2, 2, 2]];
    ///
    /// assert_eq!((2, 3), m.shape());
    /// ```
    #[inline]
    fn shape(&self) -> (usize, usize) { (self.num_rows(), self.num_cols()) }

    #[deprecated(since="0.6.0", note="please use [`shape`](#method.shape) instead")]    
    /// Returns the dimensions of the matrix. It is an alias of the [`shape`](#method.shape) method.
    #[inline]
    fn dimensions(&self) -> (usize, usize) { (self.num_rows(), self.num_cols()) }

    /// Returns the number of diagonals.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [
    ///     [3, 4, 5],
    ///     [2, 3, 4],
    ///     [1, 2, 3]
    /// ];
    ///
    /// assert_eq!(5, m.num_diags());
    /// ```
    #[inline]
    fn num_diags(&self) -> usize { self.num_cols().saturating_sub(1) + self.num_rows() }

    /// Returns the length of a row.
    #[inline(always)]
    fn row_len(&self) -> usize { self.num_cols() }

    /// Returns the length of a column.
    #[inline(always)]
    fn col_len(&self) -> usize { self.num_rows() }

    /// Gives the length of a diagonal. 
    /// Returns 0 if the matrix is empty or if the diagonal 
    /// indexed by `n` does not exist.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    /// ];
    /// 
    /// assert_eq!(1, m.diag_len(0));
    /// assert_eq!(2, m.diag_len(1));
    /// assert_eq!(3, m.diag_len(2));
    /// assert_eq!(3, m.diag_len(3));
    /// assert_eq!(2, m.diag_len(4));
    /// assert_eq!(1, m.diag_len(5));
    ///
    /// assert_eq!(0, m.diag_len(6));
    ///
    /// let empty: [[u8; 0]; 1] = [[]]; 
    /// assert_eq!(0, empty.diag_len(0));
    /// ```
    fn diag_len(&self, mut n: usize) -> usize {
        let (rows, cols) = self.shape();
        // num_diags()
        let ndiags = cols.saturating_sub(1) + rows;
        if self.is_empty() || n >= ndiags {
            return 0;
        }
        let main_diag = rows - 1;
        n = if n >= main_diag {
           // Use its symmetric to calculate length.
           ndiags - n - 1
        }
        else {
            n
         };
        
        if n > cols {
            // Cut the non existing columns.
            n -= n - cols;
        }
        // +1 because diag index statts from 0.
        n + 1
    }
    
    /// Checks if the provided subscripts point to an element inside the matrix.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let m = [ 
    ///     [(0,0), (0,1)],
    ///     [(1,0), (1,1)]
    /// ];
    ///
    /// assert!(m.check(0,0));
    /// assert!(m.check(0,1));
    /// assert!(m.check(1,0));
    /// assert!(m.check(1,1));
    /// assert!(!m.check(2,0));
    /// ```
    #[inline]
    fn check(&self, i: usize, j: usize) -> bool {
        i < self.num_rows() && j < self.num_cols()
    }
    
    /// Checks if the provided linear index point to an element inside the matrix.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [ 
    ///     [0, 1],
    ///     [2, 3]
    /// ];
    ///
    /// assert!(m.check_nth(0));
    /// assert!(m.check_nth(1));
    /// assert!(m.check_nth(2));
    /// assert!(m.check_nth(3));
    /// assert!(!m.check_nth(4));
    /// ```
    #[inline]
    fn check_nth(&self, n: usize) -> bool {
        n < self.size()
    }
    
    /// Use matrix as a subscripts-to-index converter.
    ///
    /// Index provided follows the *Row Major Order*.
    ///
    /// This does not check if either the provided subscripts or the given index are out of bounds.
    /// For a safe alternative see [`checked_index_from`](#checked_index_from).
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [ 
    ///     [0, 1],
    ///     [2, 3],
    /// ];
    ///
    /// assert_eq!(0, m.index_from((0, 0)));
    /// assert_eq!(1, m.index_from((0, 1)));
    /// assert_eq!(2, m.index_from((1, 0)));
    /// assert_eq!(3, m.index_from((1, 1)));
    ///
    /// // If passing out-of-bound subscripts to the method.
    ///
    /// assert_eq!(4, m.index_from((2, 0)));
    /// assert_eq!(7, m.index_from((2, 3)));    
    /// assert_eq!(14, m.index_from((2, 10)));
    /// ```
    #[inline]
    fn index_from(&self, subscripts: (usize, usize)) -> usize {
        subscripts.0 * self.num_cols() + subscripts.1
    }

    /// Use matrix as a index-to-subscripts converter.
    ///
    /// Indexes(subscripts) are obtained from index by *Row Major Order*.
    ///
    /// This does not check if either the provided subscripts or the given index are out of bounds.
    /// For a safe alternative see [`checked_subscripts_from`](#checked_subscripts_from).
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [ 
    ///     [0, 1],
    ///     [2, 3],
    /// ];
    ///
    /// // This method visualizes an array of indexes from `0` to `n`
    /// // (= 4 for this explanation): `[0, 1, 2, 3, 4]`.
    /// // Then this array is divided into array of `m.row_len()` length
    /// // (= 2 in our case): [ 0, 1 | 2, 3 ] => 0:[0, 1], 1:[0, 1], 2:[0, ]
    /// // Finally the last subscript `(2, 0)` is returned
    ///
    /// assert_eq!((0, 0), m.subscripts_from(0));
    /// assert_eq!((0, 1), m.subscripts_from(1));
    /// assert_eq!((1, 0), m.subscripts_from(2));
    /// assert_eq!((1, 1), m.subscripts_from(3));
    ///
    /// // If passing out-of-bound index to the method.
    ///
    /// assert_eq!((2, 0), m.subscripts_from(4));
    /// assert_eq!((3, 1), m.subscripts_from(7));    
    /// assert_eq!((7, 0), m.subscripts_from(14));
    /// ```
    #[inline]
    fn subscripts_from(&self, n: usize) -> (usize, usize) { 
        (n / self.num_cols(), n % self.num_cols())
    }
    
    /// Checked index calculation.
    ///
    /// Returns None if indexes are out of bounds of the matrix.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [ 
    ///     [0, 1],
    ///     [2, 3],
    /// ];
    ///
    /// assert_eq!(Some(0), m.checked_index_from((0, 0)));
    /// assert_eq!(Some(1), m.checked_index_from((0, 1)));
    /// assert_eq!(Some(2), m.checked_index_from((1, 0)));
    /// assert_eq!(Some(3), m.checked_index_from((1, 1)));
    ///
    /// assert_eq!(None, m.checked_index_from((2, 0)));
    /// ```
    #[inline]
    fn checked_index_from(&self, subscripts: (usize, usize)) -> Option<usize> {
        if self.check(subscripts.0, subscripts.1) {
            let n = subscripts.0 * self.num_cols() + subscripts.1;
            Some(n)
        }
        else {
            None
        }
    }

    /// Checked indexes calculation.
    ///
    /// Returns None if index is out of bound of the vector representation.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [ 
    ///     [0, 1],
    ///     [2, 3],
    /// ];
    ///
    /// assert_eq!(Some((0, 0)), m.checked_subscripts_from(0));
    /// assert_eq!(Some((0, 1)), m.checked_subscripts_from(1));
    /// assert_eq!(Some((1, 0)), m.checked_subscripts_from(2));
    /// assert_eq!(Some((1, 1)), m.checked_subscripts_from(3));
    ///
    /// assert_eq!(None, m.checked_subscripts_from(4));
    /// ```
    #[inline]
    fn checked_subscripts_from(&self, n: usize) -> Option<(usize, usize)> { 
        if n >= self.size() {
            None
        }
        else {
            Some((n / self.num_cols(), n % self.num_cols()))
        }
    }
 
    
    /// Returns an iterator over the elements of the matrix.
    ///
    /// Iteration follows the *Row Major Order*.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let x = &[
    ///      [1, 2],
    ///      [3, 4]
    /// ];
    /// let mut iterator = x.iter();
    /// 
    /// assert_eq!(iterator.next(), Some(&1));
    /// assert_eq!(iterator.next(), Some(&2));
    /// assert_eq!(iterator.next(), Some(&3));
    /// assert_eq!(iterator.next(), Some(&4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    fn iter(&self) -> Iter<'_, Self> where Self: Sized { Iter::new(self) }

    
    /// Returns an iterator over the elements of the `i`-th row.
    ///
    /// None is returned if `i >= number of rows`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = &[[1, 2], [3, 4], [5, 6]];
    ///
    /// let mut row = m.row(2).unwrap();
    ///
    /// assert_eq!(Some(&5), row.next());
    /// assert_eq!(Some(&6), row.next());
    /// assert_eq!(None, row.next());
    /// 
    /// assert!(m.row(3).is_none());
    /// ```
    #[inline]
    fn row(&self, i: usize) -> Option<Row<'_, Self>>
    where Self: Sized
    {
        if i >= self.num_rows() {
            None
        }
        else {
            Some(Row::new(self, i))
        }
    }

    /// Returns an iterator over the elements of the `i`-th row, without doing bound checking.
    unsafe fn row_unchecked(&self, i: usize) -> Row<'_, Self> 
    where Self: Sized
    {
       self.row(i).unwrap_unchecked()
    }    
    
    /// Returns an iterator over elements of the `j`-th column.
    ///
    /// None is returned if `j >= number of columns`.    
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = &[[1, 2], [3, 4], [5, 6]];
    ///
    /// let mut col = m.col(1).unwrap();
    ///
    /// assert_eq!(Some(&2), col.next());
    /// assert_eq!(Some(&4), col.next());
    /// assert_eq!(Some(&6), col.next());
    /// assert_eq!(None, col.next());
    ///
    /// assert!(m.col(2).is_none());    
    /// ```
    #[inline]
    fn col(&self, j: usize) -> Option<Column<'_, Self>> 
    where Self: Sized
    {
        if j >= self.num_cols() {
            None
        }
        else {
            Some(Column::new(self, j))
        }
    }

    /// Returns an iterator over the elements of the `j`-th column, without doing bound checking.
    unsafe fn col_unchecked(&self, j: usize) -> Column<'_, Self> 
    where Self: Sized
    {
        self.col(j).unwrap_unchecked()
    }    
    
    /// Returns an iterator over element of the `n`-th diagonal of the matrix,
    /// starting from bottom-left to top-right.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let mut m = &[
    ///     [1, 4, 6],
    ///     [7, 2, 5],
    ///     [9, 8, 3]
    /// ];
    /// 
    /// let mut diag = m.diag(3).unwrap();
    /// assert_eq!(Some(&4), diag.next());
    /// assert_eq!(Some(&5), diag.next());
    /// assert_eq!(None, diag.next());
    ///
    /// assert!(m.diag(5).is_none());
    /// ```
    #[inline]
    fn diag(&self, n: usize) ->  Option<Diag<'_, Self>>
    where Self: Sized
    {
        if n >= self.num_diags() {
            None
        }
        else {
            Some(Diag::new(self, n))
        }
    }
    
    /// Returns an iterator over the elements of the `n`-th diagonal, without doing bound checking.
    unsafe fn diag_unchecked(&self, n: usize) -> Diag<'_, Self> 
    where Self: Sized
    {
        self.diag(n).unwrap_unchecked()
    }
    
    /// Returns the main diagonal i.e. all elements at position `(i, i)`.
    //
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let mut m = &[
    ///     [1, 4, 6],
    ///     [7, 2, 5],
    ///     [9, 8, 3]
    /// ];
    ///
    /// let mut diag = m.main_diag();
    /// 
    /// assert_eq!(Some(&1), diag.next());
    /// assert_eq!(Some(&2), diag.next());
    /// assert_eq!(Some(&3), diag.next());
    /// assert_eq!(None, diag.next());
    /// ```
    fn main_diag(&self) -> Diag<'_, Self> 
    where Self: Sized {
        let n = ::core::cmp::min(self.num_rows(), self.num_cols());
        Diag::new(self, n.saturating_sub(1))
    }
    
    /// Returns an iterator which gives the current subscripts of the current element as well as its value.
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = &[[1, 2], [3, 4], [5, 6]];
    /// let mut en = m.enumerate();
    /// 
    /// assert_eq!(Some((0, 0, &1)), en.next());
    /// assert_eq!(Some((0, 1, &2)), en.next());
    /// assert_eq!(Some((1, 0, &3)), en.next());
    /// assert_eq!(Some((1, 1, &4)), en.next());
    /// assert_eq!(Some((2, 0, &5)), en.next());
    /// assert_eq!(Some((2, 1, &6)), en.next());
    /// assert_eq!(None, en.next());
    ///```
    fn enumerate(&self) -> Enumerator<Iter<'_, Self>>
    where Self: Sized
    {
        let cols = self.num_cols();
        Enumerator::new(self.iter(), cols)
    }

    /// Returns an iterator over the rows with immutable access to elements.
    ///```rust
    /// use matrixable::MatrixExt;
    ///
    /// let mut m = [[1, 2], [3, 4], [5, 6]];
    /// 
    /// let mut rows = m.rows();
    /// 
    /// assert_eq!(vec![&1, &2], rows.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&3, &4], rows.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&5, &6], rows.next().unwrap().collect::<Vec<_>>());
    /// assert!(rows.next().is_none());
    ///```
    #[inline]
    fn rows(&self) -> Rows<Self> where Self: Sized { 
        Rows::from(self)
    }

    /// Returns an iterator over the columns with immutable access to elements.
    /// ```rust    
    /// use matrixable::MatrixExt;
    ///
    /// let mut m = [[1, 2], [3, 4], [5, 6]];
    /// 
    /// let mut cols = m.cols();
    /// 
    /// assert_eq!(vec![&1, &3, &5], cols.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&2, &4, &6], cols.next().unwrap().collect::<Vec<_>>());
    /// assert!(cols.next().is_none());
    ///```
    #[inline]
    fn cols(&self) -> Columns<Self> where Self: Sized { 
        Columns::from(self)
    }
    
    /// Returns an iterator over the diagonals with immutable access to elements.
    /// Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [
    ///     [0, 1, 2],
    ///     [3, 4, 5],
    ///     [6, 7, 8]
    /// ];
    /// 
    /// let mut diags = m.diags();
    ///
    /// {
    ///     let mut first_diag = diags.next().unwrap();
    ///     assert_eq!(Some(&6), first_diag.next());
    ///     assert_eq!(None, first_diag.next());
    /// }
    ///
    /// {
    ///     let mut diag = diags.next().unwrap();
    ///     assert_eq!(Some(&3), diag.next());
    ///     assert_eq!(Some(&7), diag.next());
    ///     assert_eq!(None, diag.next());
    /// }
    /// 
    /// {
    ///     let mut diag = diags.next().unwrap();
    ///     assert_eq!(Some(&0), diag.next());
    ///     assert_eq!(Some(&4), diag.next());
    ///     assert_eq!(Some(&8), diag.next());
    ///     assert_eq!(None, diag.next());
    /// }
    ///
    /// {
    ///     let mut diag = diags.next().unwrap();
    ///     assert_eq!(Some(&1), diag.next());
    ///     assert_eq!(Some(&5), diag.next());
    ///     assert_eq!(None, diag.next());
    /// }
    /// 
    /// {
    ///     let mut last_diag = diags.next().unwrap();
    ///     assert_eq!(Some(&2), last_diag.next());
    ///     assert_eq!(None, last_diag.next());
    /// }
    ///
    /// assert!(diags.next().is_none());
    /// ```
    #[inline]
    fn diags(&self) -> Diags<Self> where Self: Sized {
        Diags::from(self) 
    }

    /// Returns an array of elements that are one-cell-adjacent to the hypothetic element located
    /// at `i`-th row and `j`-th column.
    /// # Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let m = [
    ///     [ 0,  1,  2,  3,  4],
    ///     [ 5,  6,  7,  8,  9],
    ///     [10, 11, 12, 13, 14],
    /// ];
    /// assert_eq!([Some(&1), Some(&2), Some(&3), Some(&6), Some(&8), Some(&11), Some(&12), Some(&13)], m.neighbours(1, 2));
    /// assert_eq!([None, Some(&0), Some(&1), None, Some(&6), None, Some(&10), Some(&11)], m.neighbours(1, 0));
    /// assert_eq!([Some(&8), Some(&9), None, Some(&13), None, None, None, None], m.neighbours(2, 4));
    /// ```
    /// Neighbouring elements are returned from top left to bottom right
    /// as shown is the following examples:
    /// ```plain
    /// * `@` refers to the element at (i, j)
    /// * `-` refers to other elements of the matrix that are not be returned by the function
    /// * Numbers followed by a `?` are out of the matrix. They will be mapped to `None`.
    ///  
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+
    /// | - | - | - | - | - |     | - | - | - | 0 | 1 | 2?      | - | - | - | - | - |  
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | 0 | 1 | 2 | - |     | - | - | - | 3 | @ | 4?      | - | - | - | - | - |     
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | 3 | @ | 4 | - |     | - | - | - | 5 | 6 | 7?      | - | - | - | - | - |          
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | 5 | 6 | 7 | - |     | - | - | - | - | - |         | - | - | - | 0 | 1 | 2?
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+
    /// | - | - | - | - | - |     | - | - | - | - | - |         | - | - | - | 3 | @ | 4?
    /// +---+---+---+---+---+     +---+---+---+---+---+         +---+---+---+---+---+    
    ///                                                                       5?  6?  7? 
    /// ```
    /// # Note
    /// It is not mandatory for @ to be at a correct position of the matrix.
    /// This allows some fancy ways of collecting elements at bottom or right.
    /// 
    /// ```plain
    /// +---+---+---+---+---+         +---+---+---+---+---+
    /// | - | - | - | - | - |         | - | - | - | - | 0 | 1?  2?  
    /// +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | - | - | - | - |         | - | - | - | - | 3 | @   4?
    /// +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | - | - | - | - |         | - | - | - | - | 5 | 6?  7?          
    /// +---+---+---+---+---+         +---+---+---+---+---+       
    /// | - | 0 | 1 | 2 | - |         | - | - | - | - | - |
    /// +---+---+---+---+---+         +---+---+---+---+---+
    ///       3?  @  4?
    ///                                       
    ///       5?  6? 7?
    /// ```
    #[inline]
    fn neighbours(&self, i: usize, j: usize) -> [Option<&Self::Element>; 8] {
        [
            if i.checked_sub(1).is_some() && j.checked_sub(1).is_some() {
                self.get(i-1, j-1)
            } else { None },

            if i.checked_sub(1).is_some() {
                self.get(i-1, j)
            } else { None },

            if i.checked_sub(1).is_some() {
                self.get(i-1, j+1)
            } else { None },

            if j.checked_sub(1).is_some() {
                self.get(i, j-1)
            } else { None },

            self.get(i, j+1),
            
            if j.checked_sub(1).is_some() {
                self.get(i+1, j-1)
            } else { None },

            self.get(i+1, j),
            
            self.get(i+1, j+1),
        ]
    }


    /// Creates a matrix to access elements of this matrix following an `AccessStrategy`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    /// use matrixable::strategies::ShiftFront;
    ///
    /// let m = [[0, 1], [2, 3]];
    /// let access = m.access(ShiftFront(3));
    ///
    /// assert_eq!(Some(&1), access.get(0, 0));
    /// assert_eq!(Some(&2), access.get(0, 1));
    /// assert_eq!(Some(&3), access.get(1, 0));
    /// assert_eq!(Some(&0), access.get(1, 1));
    /// ```
    /// This method returns an `Access` struct that implements `MatrixExt`.
    /// So by repeating this method on that struct you can chain access 
    /// and obtain a more complex access.
    /// ```rust
    /// use matrixable::MatrixExt;
    /// use matrixable::strategies::{ ShiftFront, FlipH, Transpose};
    ///
    /// let m = [[0, 1], [2, 3]]; 
    ///
    /// let shift = m.access(ShiftFront(3)); // [[1, 2], [3, 0]]
    /// let trans_shift = shift.access(Transpose); // [[1, 3], [2, 0]]
    /// let flip_trans_shift = trans_shift.access(FlipH); // [[3, 1], [0, 2]]
    ///
    /// assert_eq!(Some(&3), flip_trans_shift.get(0, 0));
    /// assert_eq!(Some(&1), flip_trans_shift.get(0, 1));
    /// assert_eq!(Some(&0), flip_trans_shift.get(1, 0));
    /// assert_eq!(Some(&2), flip_trans_shift.get(1, 1));
    /// ```
    /// However, prefer using [`AccessStrategySet`] method if you have a considerable number of `AccessStrategy`s to chain.
    ///
    /// [`AccessStrategySet`]: crate::strategies::AccessStrategySet
    #[inline]
    fn access<S: AccessStrategy<Self>>(&self, strategy: S) -> Access<'_, Self, S>
    where Self: Sized {
        Access::new(self, strategy)
    }
    
    
    /// Converts a matrix into an iterator over rows of the matrix.
    /// # Important
    /// Struct using this method must ensure that `IntoIterator` implementation is an iteration over
    /// **rows**, each of which implements `IntoIterator` over its elements.
    /// This requirement is indispensable for a correct use of this method.
    /// # Example
    /// ```
    /// use matrixable::MatrixExt;
    /// 
    /// let m = [[1, 2, 3], [4, 5, 6]];
    /// 
    /// let mut rows = m.into_rows();
    /// 
    /// assert_eq!(Some(vec![1, 2, 3]), rows.next());
    /// assert_eq!(Some(vec![4, 5, 6]), rows.next());
    /// 
    /// assert!(rows.next().is_none());
    /// ```
    #[inline]
    fn into_rows(self) -> IntoAxes<Self::Element>
        where Self: Sized +  IntoIterator,
              <Self as IntoIterator>::Item: IntoIterator<Item = Self::Element>
    {
        IntoAxes::from_as_rows(self)
    }

    /// Converts a matrix into an iterator over columns of the matrix.
    /// # Important
    /// Struct using this method must ensure that `IntoIterator` implementation is an iteration over
    /// **rows**, each of which implements `IntoIterator` over its elements.
    /// This requirement is indispensable for a correct use of this method.
    /// # Example
    /// ```
    /// use matrixable::MatrixExt;
    /// 
    /// let m = [[1, 2, 3], [4, 5, 6]];
    /// 
    /// let mut cols = m.into_cols();
    /// 
    /// assert_eq!(Some(vec![1, 4]), cols.next());
    /// assert_eq!(Some(vec![2, 5]), cols.next());
    /// assert_eq!(Some(vec![3, 6]), cols.next());
    /// 
    /// assert!(cols.next().is_none());
    /// ```
    #[inline]
    fn into_cols(self) -> IntoAxes<Self::Element>
        where Self: Sized + IntoIterator,
              <Self as IntoIterator>::Item: IntoIterator<Item = Self::Element>
    {
        IntoAxes::from_as_cols(self)
    }

    /// Converts a matrix into an iterator over diagonals of the matrix.
    /// # Important
    /// Struct using this method must ensure that `IntoIterator` implementation is an iteration over
    /// **rows**, each of which implements `IntoIterator` over its elements.
    /// This requirement is indispensable for a correct use of this method.
    /// # Example
    /// ```
    /// use matrixable::MatrixExt;
    /// 
    /// let m = [
    ///     [1, 2, 3],
    ///     [4, 5, 6]
    /// ];
    /// 
    /// let mut diags = m.into_diags();
    /// 
    /// assert_eq!(Some(vec![4]), diags.next());
    /// assert_eq!(Some(vec![1, 5]), diags.next());
    /// assert_eq!(Some(vec![2, 6]), diags.next());
    /// assert_eq!(Some(vec![3]), diags.next());
    /// 
    /// assert!(diags.next().is_none());
    /// ```
    #[inline]
    fn into_diags(self) -> IntoAxes<Self::Element>
    where Self: Sized + IntoIterator,
          <Self as IntoIterator>::Item: IntoIterator<Item = Self::Element>
    {
        IntoAxes::from_as_diags(self)
    }

    /// Consumes the matrix an returns an output defined by a `TransformStrategy`.
    #[inline] 
    fn transform<S: TransformStrategy<Self>>(self, strategy: &S) -> S::Output  
    where Self: Sized
    {
        strategy.out_of(self)
    }

    /// Returns the subscripts of the first element of the matrix that matches the condition.
    #[inline]
    fn position2(&self, f: impl Fn(&Self::Element)->bool) -> Option<(usize, usize)> {
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
                if f(self.get(i, j).unwrap()) {
                   return Some((i, j))
                }
            }
        }

        None
    }

    /// Checks if the matrix is empty.
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert!(![[0]].is_empty());
    /// assert!(![[0], [0]].is_empty());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(empty.is_empty());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(empty2.is_empty());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(empty3.is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    /// Checks if the matrix is a square matrix (a matrix with equal number of rows and columns).
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// // singleton
    /// assert!([[1]].is_square());
    /// 
    /// // row
    /// assert!(![[1, 2, 3]].is_square());
    /// 
    /// // column
    /// assert!(![[0], [1], [3]].is_square());
    /// 
    /// // square
    /// assert!([[0; 4]; 4].is_square());
    ///
    /// // All those three are valid because they are all empty matrices.
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(empty.is_square());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(empty2.is_square());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(empty3.is_square());
    /// 
    /// // any other
    /// assert!(![[0; 2]; 4].is_square());
    /// ```
    #[inline]
    fn is_square(&self) -> bool {
        self.num_rows() == self.num_cols()
    }

    
    /// Checks if the matrix is a vector (number of columns is `1` or number of rows is `1`)
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert_eq!(true, [[0]].is_vector());
    /// assert_eq!(true, [[0, 0]].is_vector());
    /// assert_eq!(true, [[0], [0]].is_vector());
    /// assert_eq!(false, [[0, 0], [0, 0]].is_vector());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert_eq!(false, empty.is_vector());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert_eq!(false, empty2.is_vector());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert_eq!(false, empty3.is_vector());
    /// ```
    #[inline]
    fn is_vector(&self) -> bool {
        self.num_rows() == 1 || self.num_cols() == 1
    }

    #[deprecated(since="0.1.2", note="please use `is_vector` instead")]
    fn is_one_dimension(&self) -> bool {
        self.num_rows() == 1 || self.num_cols() == 1
    }

    /// Checks if the matrix is symmetric i.e. it does not change when transposed.
    /// 
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert!([[0]].is_symmetric());
    /// assert!([[1, 0, 0], [0, 1, 0], [0, 0, 1]].is_symmetric());
    /// assert!([[1], [2], [3]].is_symmetric());
    /// assert!(![[1, 2], [2, 3], [3, 4]].is_symmetric());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(!empty.is_symmetric());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(!empty2.is_symmetric());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(!empty3.is_symmetric());
    /// ```
    fn is_symmetric(&self) -> bool
    where
        Self::Element: PartialEq
    {
        let r = self.num_rows();
        let c = self.num_cols();
        
        if self.is_empty() { 
            return false
        }
        
        let limit = r * c  -  1;

        let mut hash = alloc::vec::Vec::new();

        let mut dest: usize;
        for n in 1..limit {
            dest = (n * r) % limit;

            if hash.contains(&n) {
                continue;
            }

            hash.push(dest);

            let (i, j) = (n / c, n % c);
            let (i_dest, j_dest) = (dest / c, dest % c);
            
            if self.get(i, j).unwrap() != self.get(i_dest, j_dest).unwrap() {
                return false
            }
        }

        true
    }
    
    /// Checks if the matrix is skew-symmetric (antisymmetric).
    /// 
    /// # Example
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m1: [[i8; 3]; 3] = [
    ///     [ 0, -1, -2 ],
    ///     [ 1,  0,  5 ],
    ///     [ 2, -5,  0 ]
    /// ];
    ///   
    /// assert!(!m1.is_symmetric());
    /// assert!(m1.is_skew_symmetric());
    ///
    /// let m2: [[i8; 3]; 3] = [
    ///     [ 0,  1,  2 ],
    ///     [ 1,  0,  1 ],
    ///     [ 2,  1,  0 ]
    /// ];
    ///   
    /// assert!(m2.is_symmetric());
    /// assert!(!m2.is_skew_symmetric());
    /// ```
    fn is_skew_symmetric(&self) -> bool 
    where 
        Self: Sized,
        for<'a> &'a Self::Element: ::core::ops::Neg,
        for<'a> Self::Element: ::core::cmp::PartialEq<<&'a Self::Element as ::core::ops::Neg>::Output>
    {
        use ::core::ops::Neg;
        self.access(crate::strategies::Transpose).iter()
            .zip(self.iter())
            .all(|(x, y)| *x == y.neg())
    }

    /// Checks if the matrix is a singleton i.e. dimensions are equal to `(1, 1)`.
    ///
    /// # Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert!([[0]].is_singleton());
    /// assert!(![[0],[0]].is_singleton());
    /// assert!(![[0,0]].is_singleton());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(!empty.is_singleton());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(!empty2.is_singleton());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(!empty3.is_singleton());
    /// ```
    #[inline]
    fn is_singleton(&self) -> bool {
        self.shape() == (1, 1)
    }
    
    
    /// Checks if the matrix is horizontal (number of rows of the matrix is lower than number of columns).
    ///
    /// # Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert!([[0]].is_horizontal());
    /// assert!([[0,0]].is_horizontal());
    /// assert!(![[0],[0]].is_horizontal());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(empty.is_horizontal());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(empty2.is_horizontal());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(empty3.is_horizontal());
    /// ```
    #[inline]
    fn is_horizontal(&self) -> bool {
        self.num_rows() <= self.num_cols()
    }
    
    /// Checks if the matrix is vertical (number of rows of the matrix is greater than number of columns).    
    ///
    /// # Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// assert!([[0]].is_vertical());
    /// assert!([[0],[0]].is_vertical());
    /// assert!(![[0,0]].is_vertical());
    ///
    /// let empty: [[u8; 0]; 0] = [];
    /// assert!(empty.is_vertical());
    ///
    /// let empty2: [[u8; 0]; 1] = [[]];
    /// assert!(empty2.is_vertical());
    ///
    /// let empty3: [[u8; 0]; 2] = [[], []];
    /// assert!(empty3.is_vertical());
    /// ```
    #[inline]
    fn is_vertical(&self) -> bool {
        self.num_rows() >= self.num_cols()
    }
    
    /// Returns a boolean indicating if the matrix looks like a diagonal matrix (a matrix which entries outside the main diagonal are all zero), along with the reference to the element that may serve as zero in that matrix if the check was correct.
    ///
    /// # Examples
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m = [
    ///     [1, 0, 0],
    ///     [0, 2, 0],
    ///     [0, 0, 3]
    /// ];
    /// assert_eq!((true, Some(&0)), m.is_diagonal());
    ///
    /// assert_eq!((true, None), [[1]].is_diagonal());
    ///
    /// assert_eq!((false, None), [[1],[0],[2]].is_diagonal());    
    /// ``` 
    fn is_diagonal(&self) -> (bool, Option<&Self::Element>) 
    where 
        Self: Sized,
        for<'a> &'a Self::Element: PartialEq
    {
        let r#false = (false, None);
        
        if self.is_singleton() {
            return (true, None)
        }
        
        // A second element must exist if matrix is not a singleton.
        // Index (0, 1) or (1, 0): not on the main diagonal and must be the same value everywhere except on that diagonal
        let zero = if let Some(z) = self.get(0, 1) {
            z 
        } else {
            self.get(1, 0).expect("Second element either from row or column")
        };
        
        for (i, j, el) in self.enumerate() {    
            if i == j {
                if el == zero {
                    return r#false
                }
            }
            else if el != zero { 
                return r#false
            }
        }
        (true, Some(zero)) 
    }     

    /// Returns a boolean indicating if matrix is a **square diagonal matrix** having the 
    /// same elements on its diagonal (assumed to be the first element of the matrix, at (0, 0)),
    /// along with that element and the element considered as zero (that is the second element of matrix, at index `0, 1`).
    ///
    /// # Examples 
    /// ```rust
    /// use matrixable::MatrixExt;
    ///
    /// let m1 = [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0]
    /// ];
    ///
    /// let mut m2 = [
    ///     [1, 0, 0],
    ///     [0, 2, 0],
    ///     [0, 0, 3]
    /// ];
    ///
    /// // rectangular matrix is not scalar...
    /// assert_eq!([
    ///         [1, 0, 0],
    ///         [0, 2, 0]
    ///     ].is_scalar(),
    ///    (false, None, None)
    /// );
    ///
    /// assert_eq!(m1.is_scalar(), (false, Some(&0), Some(&0)));
    /// assert_eq!(m2.is_scalar(), (false, Some(&1), Some(&0)));
    ///
    /// m2[1][1] = 1;
    /// m2[2][2] = 1;
    ///
    /// assert_eq!(m2.is_scalar(), (true, Some(&1), Some(&0)));
    /// ```
    fn is_scalar(&self) -> (bool, Option<&Self::Element>, Option<&Self::Element>) 
    where 
        Self: Sized,
        for<'a> &'a Self::Element: PartialEq,
    {
        if !self.is_square()  { 
            return (false, None, None)
        }
        
        // Here we assume that a singleton matrix is always scalar.
        if self.is_singleton() {
            return (true, self.get(0,0), None)
        }
        
        let one = self.get(0,0).expect("First element.");
        
        // index (0, 1) or (1, 0): not on the main diagonal and must be the same value everywhere except on that diagonal
        let zero = if let Some(z) = self.get(0,1) {
            z 
        } else {
            self.get(1,0).expect("Second element either from row or column")
        };
        
        if one == zero { 
            return (false, Some(one), Some(zero))
        }
        
        for (i, j, el) in self.enumerate() {
            if i == j {
                if el == zero || el != one {
                    return (false, Some(one), Some(zero))
                }
            }
            else if el != zero { 
                return (false, Some(one), Some(zero))                }
        }
        
        (true, Some(one), Some(zero)) 
    }
    
    /// Returns a boolean indicating if all elements of the matrix are equal,
    /// and that element if it the check value is `true`.
    ///
    /// # Examples 
    /// ```rust
    /// use matrixable::MatrixExt;
    /// 
    /// let mut m = [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0]
    /// ];
    /// 
    /// assert_eq!(m.is_constant(), (true, Some(&0)));
    /// m[0][2] = 5;
    ///
    /// assert_eq!(m.is_constant(), (false, None));
    ///
    /// // All elements are now equal to five.
    /// m.iter_mut().flatten().for_each(|x| *x = 5);
    ///
    /// assert_eq!(m.is_constant(), (true, Some(&5)));
    /// ```
    fn is_constant(&self) -> (bool, Option<&Self::Element>)
    where Self::Element: PartialEq {
        if self.is_empty() {
            return (false, None)
        }
        
        let el = self.get(0,0).unwrap();
        
        for i in 0..self.num_rows() {
            for j in 0..self.num_cols() {
               if self.get(i, j).unwrap() != el {
                    return (false, None)
               }
            }
        }
        
        (true, Some(el))
    }
}
 
/// This trait adds mutable access and some additional methods to [`MatrixExt`] implementors.
pub trait MatrixMutExt: MatrixExt {
    // Required
    
    /// Returns a mutable reference to a value inside the matrix, at the intersection of the `i`-th row and the `j`-th column.
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element>;
    
    
    // Provided
    
    /// Returns a mutable reference to an element, without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_mut(row_index, column_index).unwrap_unchecked()`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let x = &mut [[1, 2, 4]];
    ///
    /// unsafe {
    ///     let elem = x.get_unchecked_mut(0, 1);
    ///        *elem = 13;
    /// }
    ///
    /// assert_eq!(x, &[[1, 13, 4]]);
    /// ```
    /// [`get_mut`]: crate::MatrixMutExt::get_mut
    unsafe fn get_unchecked_mut(&mut self, row: usize, column: usize) -> &mut Self::Element {
        self.get_mut(row, column).unwrap_unchecked()
    }
    
    
    // Gets a mutable reference to an element inside a matrix, given its order of disposition in *Row Major Order*.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut v = [[0, 1, 3, 3, 4, 5]];
    ///
    /// assert_eq!(3, v[0][2]);
    ///
    /// let n = v.get_nth_mut(2).unwrap();
    /// *n = 2;
    ///
    /// assert_eq!(2, v[0][2]);
    /// ```
    #[inline]
    fn get_nth_mut(&mut self, n: usize) -> Option<&mut Self::Element> {
        let (i, j) = self.subscripts_from(n);
        self.get_mut(i, j)
    }
    
    
    /// Returns mutable a reference to an element given its linear order, without doing bound checking.
    ///
    /// For a safe alternative see [`get_nth_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_nth_mut(index).unwrap_unchecked()`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let x = &mut [[1, 2, 4]];
    ///
    /// unsafe {
    ///     let elem = x.get_nth_unchecked_mut(1);
    ///        *elem = 13;
    /// }
    ///
    /// assert_eq!(x, &[[1, 13, 4]]);
    /// ```
    /// [`get_nth_mut`]: crate::MatrixMutExt::get_nth_mut
    unsafe fn get_nth_unchecked_mut(&mut self, n: usize) -> &mut Self::Element {
        let (i, j) = self.subscripts_from(n);
        self.get_unchecked_mut(i, j)
    }
    
    #[inline]
    /// Returns a mutable pointer to the first element of the  matrix, or `None` if it is empty.
    fn first_mut(&mut self) -> Option<&mut Self::Element> {
        self.get_mut(0, 0)
    }
    
    #[inline]
    /// Returns a mutable pointer to the last element of the  matrix, or `None` if it is empty.
    fn last_mut(&mut self) -> Option<&mut Self::Element> {
        match self.shape() {
            (0, _) | (_, 0) => None,
            (r, c) => self.get_mut(r - 1, c - 1)
        }
    }
    
    
    /// Changes the value of an element at the intersection of the `i`-th row and the `j`-th column of the matrix.
    ///
    /// # Error
    /// An error is returned if any of those indexes are out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::{MatrixExt, MatrixMutExt}; 
    /// 
    /// let mut m = [[1, 2, 3]];
    ///
    /// assert_eq!(Ok(()), m.set((0, 2), 100));
    /// assert_eq!(Some(&100), m.get(0, 2));
    ///
    /// assert_eq!(Err("Cannot access element from indexes."), m.set((1, 0), 11));
    /// ```
    #[inline]
    fn set(&mut self, subscripts: (usize, usize), val: Self::Element) -> Result<(), &'static str> {
        match self.get_mut(subscripts.0, subscripts.1) {
            Some(target) => {
                *target = val;
                Ok(())
            }
            None => Err("Cannot access element from indexes."),
        }
    }   
    
    /// Changes the value of the `n`-th element of the matrix.
    ///
    /// # Error
    /// An error is returned if `n` is out of bound.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::{MatrixExt, MatrixMutExt}; 
    /// 
    /// let mut m = [[1, 2, 3]];
    ///
    /// assert_eq!(Ok(()), m.set_nth(2, 100));
    /// assert_eq!(Some(&100), m.get(0, 2));
    ///
    /// assert_eq!(Err("Cannot access element from index."), m.set_nth(3, 11));
    /// ```
    #[inline]
    fn set_nth(&mut self, n: usize, val: Self::Element) -> Result<(), &'static str> {
        let (i, j) = self.subscripts_from(n);
        match self.get_mut(i, j) {
            Some(target) => {
                *target = val;
                Ok(())
            }
            None => Err("Cannot access element from index."),
        }
    }
    
    /// Swaps two elements in the matrix identified by their subscripts.
    ///
    /// If a equals to b, it’s guaranteed that elements won’t change value.
    ///
    /// # Arguments
    /// - a - The index of the first element
    /// - b - The index of the second element
    ///
    /// # Panics
    ///
    /// Panics if a or b are out of bounds.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut m = [
    ///     [(0,0), (0,1), (0,2)],
    ///     [(1,0), (1,1), (1,2)],
    ///     [(2,0), (2,1), (2,2)]
    /// ];
    /// 
    /// m.swap((0,1), (2,2));
    ///
    /// let expected = [
    ///    [(0,0), (2,2), (0,2)],
    ///    [(1,0), (1,1), (1,2)],
    ///    [(2,0), (2,1), (0,1)]
    /// ];
    ///
    /// assert_eq!(expected, m);
    /// ```
    fn swap(&mut self, a:(usize, usize), b:(usize, usize)) {
        if a == b { return }
        let a: *mut Self::Element = self.get_mut(a.0, a.1).unwrap();
        let b: *mut Self::Element = self.get_mut(b.0, b.1).unwrap();
        unsafe { ::core::ptr::swap(&mut *a, &mut *b) };
    }

    /// Swaps two elements in the matrix identified by their linear position following the *Row Major Order*.
    ///
    /// If a equals to b, it’s guaranteed that elements won’t change value.
    ///
    /// # Arguments
    ///
    /// - a - The index of the first element
    /// - b - The index of the second element
    ///
    /// # Panics
    ///
    /// Panics if a or b are out of bounds.
    ///
    /// # Example    
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut m = [
    ///     [(0,0), (0,1), (0,2)],
    ///     [(1,0), (1,1), (1,2)],
    ///     [(2,0), (2,1), (2,2)]
    /// ];
    ///
    /// m.swapn(2, 7);
    /// 
    /// let expected = [
    ///         [(0,0), (0,1), (2,1)],
    ///         [(1,0), (1,1), (1,2)],
    ///         [(2,0), (0,2), (2,2)]
    /// ];
    ///
    /// assert_eq!(expected, m);
    /// ```
    fn swapn(&mut self, a: usize, b: usize) {
        if a == b { return }
        let a: *mut Self::Element = self.get_nth_mut(a).unwrap();
        let b: *mut Self::Element = self.get_nth_mut(b).unwrap();
        unsafe { ::core::ptr::swap(&mut *a, &mut *b) };
    }

    /// Swaps two columns.
    /// # Panics
    /// Panics if a column index is out of bound.
    #[inline]
    fn swap_cols(&mut self, col1: usize, col2: usize) {
        for i in 0..self.num_rows() {
            self.swap((i, col1), (i, col2));
        }
    }


    /// Swaps two rows.
    /// # Panics
    /// Panics if a row index is out of bound.
    #[inline]
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        for j in 0..self.num_cols() {
            self.swap((row1, j), (row2, j));
        }
    }

    /// Returns an iterator that allows modifying each element.
    ///
    /// Iteration follows the *Row Major Order*.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    /// let x = &mut [[1, 2, 4], [2, 5, 6]];
    /// 
    /// let third = x.iter_mut().nth(2).unwrap();
    /// *third = 3;
    ///
    /// let fourth = x.iter_mut().nth(3).unwrap();
    /// *fourth = 4;
    ///
    /// assert_eq!(x, &mut [[1, 2, 3], [4, 5, 6]]);
    /// ```
    #[inline]
    fn iter_mut(&mut self) -> IterMut<'_, Self> where Self: Sized { IterMut::new(self) }
    
    /// Returns an iterator that allows modifying each element of the `i`-th row.
    ///
    /// None is returned if `i >= number of rows`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let x = &mut [[1, 2, 4], [2, 5, 6]];
    /// 
    /// for elem in x.row_mut(1).unwrap() {
    ///     *elem = 0;
    /// }
    /// 
    /// assert_eq!(x, &mut [[1, 2, 4], [0, 0, 0]]);
    /// ```
    #[inline]
    fn row_mut(&mut self, i: usize) -> Option<RowMut<'_, Self>> 
    where Self: Sized 
    {
        if i >= self.num_rows() {
            None
        }
        else {
            Some(RowMut::new(self, i))
        }
    }
    
    /// Returns an iterator over the mutable elements of the `i`-th row, without doing bound checking.
    unsafe fn row_unchecked_mut(&mut self, i: usize) -> RowMut<'_, Self> 
    where Self: Sized {
        self.row_mut(i).unwrap_unchecked()
    }
    
    /// Returns an iterator over that allows modifying each element of the `j`-th column.
    ///
    /// None is returned if `j >= number of columns`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let x = &mut [[1, 2, 4], [2, 5, 6]];
    /// 
    /// for elem in x.col_mut(1).unwrap() {
    ///     *elem += 3;
    /// }
    /// 
    /// assert_eq!(x, &mut [[1, 5, 4], [2, 8, 6]]);
    /// ```
    #[inline]
    fn col_mut(&mut self, j: usize) -> Option<ColumnMut<'_, Self>>
    where Self: Sized
    {
        if j >= self.num_cols() {
            None
        }
        else {
            Some(ColumnMut::new(self, j))
        }
    } 
    
    /// Returns an iterator over the mutable elements of the `j`-th column, without doing bound checking.
    unsafe fn col_unchecked_mut(&mut self, j: usize) -> ColumnMut<'_, Self>
    where Self: Sized {
        self.col_mut(j).unwrap_unchecked()
    }

    /// Returns an iterator over that allows modifying each element of the `n`-th diagonal.
    ///
    /// None is returned if `n >= number of diagonals`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let m = &mut [
    ///     [0, 0, 0],
    ///     [0, 0, 0],
    ///     [0, 0, 0]
    /// ];
    ///     
    /// for elem in m.diag_mut(2).unwrap() {    
    ///     *elem = 1;
    /// }
    /// 
    /// assert_eq!(&mut [
    ///     [1, 0, 0],
    ///     [0, 1, 0],
    ///     [0, 0, 1],
    /// ], m);
    /// ```
    #[inline]
    fn diag_mut(&mut self, n: usize) ->  Option<DiagMut<'_, Self>>
    where Self: Sized
    {
        if n >= self.num_diags() {
            None
        }
        else {
            Some(DiagMut::new(self, n))
        }
    }
    
    /// Returns an iterator over the mutable elements of the `n`-th diagonal, without doing bound checking.
    unsafe fn diag_unchecked_mut(&mut self, n: usize) -> DiagMut<'_, Self>
    where Self: Sized {
        self.diag_mut(n).unwrap_unchecked()
    }
    
    /// Returns the main diagonal (mutable).
    //
    /// # Example
    /// ```rust    
    /// use matrixable::MatrixMutExt;
    ///
    /// let m = &mut [
    ///     [0, 0],
    ///     [0, 0],
    ///     [0, 0]
    /// ];
    ///     
    /// // for elem in m.diag_mut(2).unwrap() {    
    /// for elem in m.main_diag_mut() {    
    ///     *elem = 1;
    /// }
    /// 
    /// assert_eq!(&mut [
    ///     [1, 0],
    ///     [0, 1],
    ///     [0, 0],
    /// ], m);
    /// ```
    fn main_diag_mut(&mut self) -> DiagMut<'_, Self> 
    where Self: Sized {
        let n = self.num_rows();
        DiagMut::new(self, n.saturating_sub(1))
    }
    
    /// [`.enumerate()`] with mutable access to each element.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut  m = [[1, 2], [3, 4], [5, 6]];
    /// let mut en = m.enumerate_mut();
    /// 
    /// assert_eq!(Some((0, 0, &mut 1)), en.next());
    /// assert_eq!(Some((0, 1, &mut 2)), en.next());
    /// assert_eq!(Some((1, 0, &mut 3)), en.next());
    /// assert_eq!(Some((1, 1, &mut 4)), en.next());
    /// assert_eq!(Some((2, 0, &mut 5)), en.next());
    /// assert_eq!(Some((2, 1, &mut 6)), en.next());
    /// assert_eq!(None, en.next());
    /// ```
    fn enumerate_mut(&mut self) -> Enumerator<IterMut<'_, Self>>
    where Self: Sized
    {
        let cols = self.num_cols();
        Enumerator::new(self.iter_mut(), cols)
    }
    
    /// Returns an iterator over the rows with mutable access to elements.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut m = [[1, 2], [3, 4], [5, 6]];
    /// 
    /// let mut rows = m.rows_mut();
    /// 
    /// assert_eq!(vec![&mut 1, &mut 2], rows.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&mut 3, &mut 4], rows.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&mut 5, &mut 6], rows.next().unwrap().collect::<Vec<_>>());
    /// assert!(rows.next().is_none());
    ///
    /// ```
    #[inline]
    fn rows_mut(&mut self) -> RowsMut<Self> where Self: Sized {
        RowsMut::from(self) 
    }

    /// Returns an iterator over the columns of the matrix with mutable access to elements.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    ///
    /// let mut m = [[1, 2], [3, 4], [5, 6]];
    /// 
    /// let mut cols = m.cols_mut();
    /// 
    /// assert_eq!(vec![&mut 1, &mut 3, &mut 5], cols.next().unwrap().collect::<Vec<_>>());
    /// assert_eq!(vec![&mut 2, &mut 4, &mut 6], cols.next().unwrap().collect::<Vec<_>>());
    /// assert!(cols.next().is_none());
    /// ```
    #[inline]
    fn cols_mut (&mut self) -> ColumnsMut<Self> where Self: Sized {
        ColumnsMut::from(self) 
    }
    
    /// Returns an iterator over the diagonals with mutable access to elements.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    /// 
    /// let mut m = [[0, 0, 0]; 3];
    ///
    /// let mut i = 0;
    /// for diag in m.diags_mut() {
    ///     i += 1;
    ///     for elem in diag {
    ///         *elem = i;
    ///     }
    /// }
    ///
    /// assert_eq!([
    ///     [3, 4, 5],
    ///     [2, 3, 4],
    ///     [1, 2, 3]
    /// ], m);
    /// ```
    #[inline]
    fn diags_mut (&mut self) -> DiagsMut<Self> where Self: Sized {
        DiagsMut::from(self) 
    }
    
    /// Creates a matrix to mutably access elements of this matrix following an `AccessStrategy`.
    ///
    /// # Example
    /// ```rust
    /// use matrixable::MatrixMutExt;
    /// use matrixable::strategies::{AccessStrategy, Reverse};
    ///
    /// let mut m = [[1, 2], [3, 4]];
    /// 
    /// m.access_mut(Reverse).set((0,0), 11).unwrap();
    ///
    /// assert_eq!([[1, 2], [3, 11]], m);
    /// ```
    /// By repeating this method you can obtain a more complex access.
    /// ```rust
    /// use matrixable::{MatrixMutExt};
    /// use matrixable::strategies::{Reverse, ShiftBack};
    ///  
    /// let mut m = [[1, 2, 3, 4], [5, 6, 7, 8]];
    ///
    /// m.access_mut(Reverse) // [[8, 7, 6, 5], [4, 3, 2, 1]]
    ///  .access_mut(ShiftBack(5))  // [[3, 2, 1, 8], [7, 6, 5, 4]]
    ///  .col_mut(0) // [3, 7]
    ///  .unwrap()
    ///  .for_each(|x| *x = 11);
    ///
    /// assert_eq!([[1, 2, 11, 4], [5, 6, 11, 8]], m);
    /// ```
    /// However, prefer using [`AccessStrategySet`] method 
    /// if you have a considerable number of `AccessStrategy`s to chain.
    ///
    /// [`AccessStrategySet`]: crate::strategies::AccessStrategySet
    #[inline]
    fn access_mut<S: AccessStrategy<Self>>(&mut self, strategy: S) -> AccessMut<'_, Self, S>
    where Self: Sized {
        AccessMut::new(self, strategy)
    }
    
    /// Modifies the matrix [`InPlace`] according to a certain strategy. 
    #[inline]
    fn in_place<S: InPlace<Self>>(&mut self, strategy: S)
    where Self: Sized {
        strategy.in_place(self)
    }

}
