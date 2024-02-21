//! This library provides a trait that implements a matrix as well as a growable generic matrix for containing and manipulating elements of any type.
//!
//! A matrix implementing Matrix is by default in *[`Row Major Order`]*, but you can change it using transpose access, at almost no cost. 
//!
//! [`Row Major Order`]: https://wikipedia.org/Row_major_order.htm

pub mod access;
pub mod iterators;
pub mod traits;
pub mod view;


pub mod prelude {
    pub use crate::iterators::*;
    pub use crate::view::*;
    pub use crate::access::*;
    pub use crate::traits::*;
}

mod tests;

use crate::traits::Matrix;

pub fn print_rows_debug<M: Matrix> (p: &M) where <M as Matrix>::Element: std::fmt::Debug {
    println!("Rows"); 
    p.rows().for_each(|row| println!("{:?}", row.collect::<Vec<_>>()))
}

pub fn print_columns_debug<M: Matrix> (p: &M) where <M as Matrix>::Element: std::fmt::Debug {
    println!("Columns");
    p.columns().for_each(|col| println!("{:?}", col.collect::<Vec<_>>()))
}

pub fn print_diagonals_debug<M: Matrix> (p: &M) where <M as Matrix>::Element: std::fmt::Debug {
    println!("Diagonals");
    p.diagonals().for_each(|col| println!("{:?}", col.collect::<Vec<_>>()))
}

/// Helper function for converting indices into an index to be used by a slice.
pub const fn index_from(rlen: usize, i: usize, j: usize) -> usize { i * rlen + j }

/// Helper function for guessing the correct indices of the matrix matching a given index of a slice.
pub const fn indexes_from(rlen: usize, n: usize) -> (usize, usize) { (n / rlen, n % rlen) }


#[inline(always)]
pub(crate) const fn panic_if_bad_size(rows: usize, cols: usize) {
    if rows == 0 || cols == 0 {
        panic!("matrix dimension cannot be zero")
    }
}


/// This macro helps to create a new matrix from the given parameters.
#[macro_export]
macro_rules! matrix {
    [ $( [ $( $x:expr $(,)?)+ ]$(,)? )+ ] => { $crate::prelude::MatrixView::from([$([$($x),*]),+]) };

    [ $( ( $( $x:expr ),+ ) ),+ $(,)? ] => { $crate::prelude::MatrixView::from([$([$($x),*]),+]) };

    ( $x:expr, $r:expr, $c:expr ) => { $crate::prelude::MatrixView::new($x, $r, $c) };

    [ $i:expr; $r:expr, $c:expr ] => { $crate::prelude::MatrixView::with_init($i, $r, $c) };

    [ [$i:expr; $r:expr]; $c:expr ] => { $crate::prelude::MatrixView::from([[$i; $r]; $c]) };

    ( [ $( $x:expr ),+ ]; $d:expr ) => {  $crate::prelude::MatrixView::from([[$($x),+]; $d]) };
    ( ( $( $x:expr ),+ ); $d:expr ) => {  $crate::prelude::MatrixView::from([[$($x),+]; $d]) };
}
