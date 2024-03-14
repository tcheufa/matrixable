//! This module contains structs for iterating over matrices.
//!
//! Some of these structs also implement [`Index`](::core::ops::Index) allowing you to use the `container[index]` notation.
//!
//! # Examples
//! ```rust
//! use matrixable::MatrixExt;
//!
//! let m = [
//!     ['a', 'b', 'c'],
//!     ['d', 'e', 'f'],
//!     ['g', 'h', 'i']
//! ];
//!
//! {
//!     let row = m.row(0).unwrap();
//!     assert_eq!('a', row[0]);
//!     assert_eq!('b', row[1]);
//!     assert_eq!('c', row[2]);
//! }
//! 
//! {
//!     let col = m.col(1).unwrap();
//!     assert_eq!('b', col[0]);
//!     assert_eq!('e', col[1]);
//!     assert_eq!('h', col[2]);
//! }
//! 
//! {
//!     let diag = m.diag(2).unwrap();
//!     assert_eq!('a', diag[0]);
//!     assert_eq!('e', diag[1]);
//!     assert_eq!('i', diag[2]); 
//! }
//! 
//! let m = [['r', 'i', 'o']];
//! let it = m.iter();
//! 
//! assert_eq!('r', it[0]);
//! assert_eq!('i', it[1]);
//! assert_eq!('o', it[2]); 
//! ```
//! <br/>
//!
//! It may happen you just need a unique element. In that case you will rather use [`.nth()`](::core::iter::Iterator::nth) 
//! from the standard [`Iterator`] trait.
//!
//! ```rust
//! use matrixable::MatrixMutExt;
//!
//! let mut m = [
//!     ['a', 'b', 'c'],
//!     ['d', 'e', 'f'],
//!     ['g', 'h', 'i']
//! ];
//! 
//! let h = m.row_mut(2).unwrap().nth(1).unwrap();
//! // Or 
//! //    = m.rows_mut().nth(2).unwrap().nth(1).unwrap();
//! 
//! assert_eq!(&mut 'h', h);
//! 
//! *h = 'z';
//! 
//! assert_eq!(Some(&mut 'z'), m.row_mut(2).unwrap().nth(1));
//! ```

use ::core::{
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    
};


use alloc::vec::Vec;
use alloc::boxed::Box;

use alloc::collections::VecDeque;

use crate::{MatrixExt, MatrixMutExt};

macro_rules! iter {
    (
        $(#[doc = $doc:expr] $name:ident { $($mut:ident)? } { $ptr:ident } $matrixTrait:ident $getfn:ident $($start:ident)?),* ;
        $get_ij:expr ;
        $incrfn:item ;
        $lenimpl:item ;
        $nextbackimpl:item 
    ) => {
        $(
            #[doc = $doc]
            /// See its documentation for more.
            #[derive(Hash, Debug, Copy, Clone)]
            pub struct $name<'a, M: $matrixTrait>
            where M::Element: 'a 
            {
                m: *$ptr M,
                i: usize,
                j: usize,
                _marker: PhantomData<&'a M>,
            }
        
            impl<'a, M: $matrixTrait> $name<'a, M>
            where M::Element: 'a 
            {
                pub(crate) fn new(m: &'a $($mut)? M $(, $start: usize)? ) -> Self {
                    let get_start = $get_ij;
                    let (i, j) = get_start(m $(, $start)?);
                    Self { m, i, j, _marker: PhantomData }
                }
                
                #[inline(always)]
                fn use_matrix(&$($mut)? self) -> &'a $($mut)? M {
                    unsafe { (&$($mut)? *self.m) }
                } 
                
                #[inline(always)]
                fn get(&$($mut)? self, i: usize, j: usize) -> Option<&'a $($mut)? M::Element> {
                    self.use_matrix().$getfn(i, j)
                }
                
                #[inline(always)]
                $incrfn
                
            }
            
            impl<'a, M: $matrixTrait> Iterator for $name<'a, M> 
            where M::Element: 'a 
            {
                type Item = &'a $($mut)? M::Element;
                    fn next(&mut self) -> Option<Self::Item> {
                    let (i, j) = (self.i, self.j);
                    let next = self.get(i, j)?;
                    
                    (self.i, self.j) = self.increment(self.i, self.j);

                    // SAFETY: Nothing else points to or will point to the contents of this iterator.
                    Some(next)
                }
            }
            
            impl<'a, M: $matrixTrait> FusedIterator for $name<'a, M> {}

            impl<'a, M: $matrixTrait> ExactSizeIterator for $name<'a, M> {
                $lenimpl
            }

            impl<'a, M: $matrixTrait> DoubleEndedIterator for $name<'a, M> {
                $nextbackimpl
            }
            
            
            impl<'a, M: $matrixTrait> PartialEq for $name<'a, M>
            where
                M::Element: PartialEq
            {
                fn eq(&self, other: &Self) -> bool {
                    let len = self.len();
                    if len != other.len() { 
                        return false
                    }
                    for i in 0..len {
                        if self[i] != other[i] {
                            return false
                        } 
                    }
                    true
                }
            }
            
            impl<'a, M: $matrixTrait> Eq for $name<'a, M> 
            where
                M::Element: PartialEq
            {}
            
            
            /// Allows creating 2D arrays from this iterator.
            impl<'a, M: $matrixTrait, T, F> FromIterator<$name<'a, M>> for Box<[F]>
            where
                F: ::core::ops::Deref<Target = [T]>,
                F: FromIterator<<$name<'a, M> as Iterator>::Item>,
                M::Element: 'a
            {
                fn from_iter<IntoI>(iter: IntoI) -> Self
                where
                    IntoI: IntoIterator<Item = $name<'a, M>>
                {
                    iter.into_iter().map(|c| c.collect::<F>()).collect()
                }
            } 
                        
            impl<'a, M: $matrixTrait, T, F> FromIterator<$name<'a, M>> for Vec<F>
            where
                F: ::core::ops::Deref<Target = [T]>,
                F: FromIterator<<$name<'a, M> as Iterator>::Item>,
                M::Element: 'a
            {
                fn from_iter<IntoI>(iter: IntoI) -> Self
                where
                    IntoI: IntoIterator<Item = $name<'a, M>>
                {
                    iter.into_iter().map(|c| c.collect::<F>()).collect()
                }
            }
            
            impl<'a, M: $matrixTrait> ::core::ops::Index<usize> for $name<'a, M>
            where 
                M::Element: 'a
            {
                type Output = M::Element;
                
                /// # Panics
                /// Panics if the index is out of bounds.
                fn index(&self, idx: usize) -> &Self::Output {
                    let (mut i, mut j) = (self.i, self.j);
                    
                    for _ in 0..idx {
                        (i, j) = self.increment(i, j);
                    }
                    
                    let m = unsafe { &*self.m };
                    m.get(i, j).unwrap()
                }
            }
            
            $(
                impl<'a, M: $matrixTrait> ::core::ops::IndexMut<usize> for $name<'a, M>
                where 
                    M::Element: 'a
                {
                    /// # Panics
                    /// Panics if the index is out of bounds.
                    fn index_mut(&$mut self, idx: usize) -> &mut Self::Output {
                        let (mut i, mut j) = (self.i, self.j);
                    
                        for _ in 0..idx {
                            (i, j) = self.increment(i, j);
                        }
                        
                        self.use_matrix().get_mut(i, j).unwrap()
                    }
                }
            )?
            
            unsafe impl<'a, M: $matrixTrait> Send for $name<'a, M>
            where M: Send, M::Element: Send {}
            
            unsafe impl<'a, M: $matrixTrait> Sync for $name<'a, M>
            where M: Sync, M::Element: Sync {}

        )*
    }
}


iter!{
    #[doc =
    "An iterator over the elements of the matrix.\n\n\
    This struct is created by the [`iter`](MatrixExt::iter) method on [`MatrixExt`]."]
    Iter {/*no mut */} { const } MatrixExt get,
    #[doc = 
    "An iterator over the elements of the matrix (mutable).\n\n\
    This struct is created by the [`iter_mut`](MatrixMutExt::iter_mut) method on [`MatrixMutExt`]."]
    IterMut { mut } { mut } MatrixMutExt get_mut;
    |_m: &M| (0, 0) ;
    fn increment(&self, mut i: usize, mut j: usize) -> (usize, usize) {
        j += 1;
        if j == unsafe { &*self.m }.num_cols() {
            j = 0;
            i += 1;
        }
        (i, j)
    } ;
    fn len(&self) -> usize {  unsafe {&*self.m}.size()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let (rows, cols) = m.dimensions();
        let (i, j) = (rows - self.i - 1, cols - self.j - 1);
        
        (self.i, self.j) = self.increment(self.i, self.j);
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get(i, j)
    }
}

iter!{
    #[doc = 
    "An iterator over a matrix row.\n\n\
    This struct is created by the [`row`](MatrixExt::row) method on [`MatrixExt`]."]
    Row {/*no mut */} { const } MatrixExt get row,
    #[doc = 
    "An iterator over a mutable matrix row.\n\n\
    This struct is created by the [`row_mut`](MatrixMutExt::row_mut) method on [`MatrixMutExt`]."]
    RowMut { mut } { mut } MatrixMutExt get_mut row;
    |_m: &M, row| (row, 0) ;
    fn increment(&self, i: usize, j: usize) -> (usize, usize) {  
        (i, j+1)
    } ;
    fn len(&self) -> usize {  unsafe {&*self.m}.row_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let cols = m.num_cols();
        let (i, j) = (self.i, cols - self.j - 1);
        let next = self.get(i, j)?;
        
        (self.i, self.j) = self.increment(self.i, self.j);
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        Some(next)
    }
}

iter!{
    #[doc = 
    "An iterator over a matrix column.\n\n\
    This struct is created by the [`col`](MatrixExt::col) method on [`MatrixExt`]."]
    Column {/*no mut */} { const } MatrixExt get col,
    #[doc = 
    "An iterator over a mutable matrix column.\n\n\
    This struct is created by the [`col_mut`](MatrixMutExt::col_mut) method on [`MatrixMutExt`]."]
    ColumnMut { mut } { mut} MatrixMutExt get_mut col;
    |_m: &M, col| (0, col) ;
    fn increment(&self, i: usize, j: usize) -> (usize, usize) {  
        (i + 1, j)
    } ; 
    fn len(&self) -> usize { unsafe {&*self.m}.col_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let rows = m.num_rows();
        let (i, j) = (rows - self.i - 1, self.j);
        
        (self.i, self.j) = self.increment(self.i, self.j);

        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get(i, j)
    }
}

iter!{
    #[doc =
    "An iterator over a matrix diagonal.\n\n\
    This struct is created by the [`diag`](MatrixExt::diag) method on [`MatrixExt`]."]
    Diag {/*no mut */} { const } MatrixExt get n,
    #[doc =
    "An iterator over a mutable matrix diagonal.\n\n\
    This struct is created by the [`diag_mut`](MatrixMutExt::diag_mut) method on [`MatrixMutExt`]."]
    DiagMut { mut } { mut } MatrixMutExt get_mut n;
    |m: &M, mut n| {
        let lastrow = m.num_rows() - 1;
        if n <= lastrow {
            (lastrow - n, 0) 
        } else {
            n = n - lastrow;
            (0, n)
        }
    };
    fn increment(&self, i: usize, j: usize) -> (usize, usize) {  
        (i + 1, j + 1)
    }; 
    fn len(&self) -> usize { 
        let m = unsafe { &*self.m };
        let mut j = 0;
        while let false = m.check(self.i, j) {
            j += 1;
        }
        j
    };
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let (rows, cols) = m.dimensions();
        let (i, j) = (rows - self.i - 1, cols - self.j - 1);
        
        (self.i, self.j) = self.increment(self.i, self.j);
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get(i, j)
    }
}

macro_rules! dimensional_iterator {
    (   $w:ident,
        $ptr:ident,
        { $($mut:ident)? },
        $matrixTrait:ident,
        $outElem:ty,
        $callfn:ident,
        $lenfn:ident
    ) => {
    
        #[derive(Debug)]
        pub struct $w<'a, M>
        where M: $matrixTrait,
        M::Element: 'a {
            n: usize,
            m:  *$ptr M,
            _marker: PhantomData<&'a M>
        }

        impl<'a, M> Iterator for $w<'a, M> 
        where
            M: $matrixTrait,
            M::Element: 'a
        {
            type Item = $outElem;

            fn next(&mut self) -> Option<Self::Item> {
                let next = unsafe { (&$($mut)? *self.m).$callfn(self.n) };

                self.n += 1;

                // SAFETY: Nothing else points to or will point to the contents of this iterator.
                next
            }
        }

        impl<'a, M> DoubleEndedIterator for $w<'a, M> 
        where M: $matrixTrait,
        {
            fn next_back(&mut self) -> Option<Self::Item> {
                let next = unsafe {
                    let m = (&$($mut)? *self.m);
                    let end = m.$lenfn();
                    m.$callfn(end - self.n - 1)
                };

                self.n += 1;

                // SAFETY: Nothing else points to or will point to the contents of this iterator.
                next
            }

            fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
                let next = unsafe {
                    let m = (&$($mut)? *self.m);
                    let end = m.$lenfn();
                    m.$callfn(end - self.n + n - 1)
                };

                self.n += 1;

                // SAFETY: Nothing else points to or will point to the contents of this iterator.
                next
            }
        }

        impl<'a, M> FusedIterator for $w<'a, M> 
        where M: $matrixTrait
        {  }
        
        impl<'a, M> ExactSizeIterator for $w<'a, M>
        where M: $matrixTrait {
            fn len(&self) -> usize {
                let m = unsafe { &$($mut)? *self.m };
                 m.$lenfn()
            }
        }

        impl<'a, 'b, M> From<&'b $($mut)? M> for $w<'a, M>
        where 
            'b: 'a,
            M: $matrixTrait
        {
            fn from(source: &'b $($mut)? M) -> Self { 
                Self { 
                    n: 0,
                    m: source,
                    _marker: PhantomData
                } 
            }
        }
            
        unsafe impl<'a, M: $matrixTrait> Send for $w<'a, M>
        where M: Send, M::Element: Send {}
        
        unsafe impl<'a, M: $matrixTrait> Sync for $w<'a, M>
        where M: Sync, M::Element: Sync {}
    };
}


dimensional_iterator!{ 
    Rows, const, { /* no mut */}, MatrixExt,
    Row<'a, M>,
    row, num_rows 
}

dimensional_iterator!{ 
    RowsMut, mut, { mut }, MatrixMutExt,
    RowMut<'a, M>,
    row_mut, num_rows 
}

dimensional_iterator!{ 
    Columns, const, {/* no mut */}, MatrixExt,
    Column<'a, M>,
    col, num_cols
}

dimensional_iterator!{ 
    ColumnsMut, mut, { mut }, MatrixMutExt,
    ColumnMut<'a, M>,
    col_mut, num_cols 
}

dimensional_iterator!{ 
    Diags, const, { /* no mut */ }, MatrixExt,
    Diag<'a, M>,
    diag, num_diags 
}

dimensional_iterator!{ 
    DiagsMut, mut, { mut }, MatrixMutExt,
    DiagMut<'a, M>,
    diag_mut, num_diags 
}


/// An iterator that yields an element of the a matrix-like `struct` along
/// with the subscripts of that element.
pub struct Enumerator<I> {
    iter: I,
    jmp: usize,
    i: usize,
    j: usize,
}
impl<I> Enumerator<I> {
    pub(crate) fn new(iter: I, jmp: usize) -> Self {
        Self {
            iter,
            jmp,
            i: 0,
            j: 0,
        }
    }
}
impl<I> Iterator for Enumerator<I>
where
    I: Iterator,
{
    type Item = (usize, usize, <I as Iterator>::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.i, self.j, self.iter.next()?);

        self.j += 1;

        if self.j == self.jmp {
            self.j = 0;
            self.i += 1;
        }
        
        Some(next)
    }
}
impl<I> ExactSizeIterator for Enumerator<I>
where
    I: ExactSizeIterator
{
    fn len(&self) -> usize {  self.iter.len()  }
}
impl<I> DoubleEndedIterator for Enumerator<I>
where
    I: DoubleEndedIterator + ExactSizeIterator
{
    fn next_back(&mut self) -> Option<Self::Item> { 
        let i = (self.len() / self.jmp) - self.i - 1;
        let j = self.jmp - self.j - 1;
        let next_back = (i, j, self.iter.next_back()?);

        self.j += 1;

        if self.j == self.jmp {
            self.j = 0;
            self.i += 1
        }
        
        Some(next_back)  
    }
}
impl<I: FusedIterator> FusedIterator for Enumerator<I> {}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct IntoRows<T> {
    d: Vec<T>,
    n: usize
}

pub struct IntoCols<T> {
    rows: Box<[VecDeque<T>]>,
}

// DoubleEndedIterator must not be implemented for this struct
// because of possible risks due to Iterator's next method impl.
pub struct IntoDiags<T>
{
    d: Vec<T>,
    rows: usize,
    cols: usize,
    diag_size: usize,
    row_start: usize,
    col_start: usize,
    tmp_as_last_elem: *const T,
}

impl<M: MatrixExt> From<M> for IntoRows<M::Element>
where M: IntoIterator<Item = M::Element>,
{
    fn from(value: M) -> Self {
        Self { 
            n: value.num_cols(),
            d: value.into_iter().collect()
        }
    }
}

impl<M: MatrixExt> From<M> for IntoCols<M::Element>
where M: IntoIterator<Item = M::Element>,
{
    fn from(value: M) -> Self {
        let (rows, cols) = (value.num_rows(), value.num_cols());
        let mut v = Vec::with_capacity(rows);
        let mut into_vec: Vec<_> = value.into_iter().collect();
        
        for _ in 0..rows {
            v.push(into_vec.drain(..cols).collect())
        }

        Self { rows: v.into() }
    }
}

impl<M: MatrixExt> From<M> for IntoDiags<M::Element> 
where M: IntoIterator<Item = M::Element>,
{
    fn from(value: M) -> Self {
        let (rows, cols) = (value.num_rows(), value.num_cols());
        Self {
            rows,
            cols,
            row_start: rows - 1,
            col_start: 0,
            diag_size: 1,
            tmp_as_last_elem: value.get(0, cols - 1).unwrap(),
            d: value.into_iter().collect(),
        }
    }
}

impl<T> Iterator for IntoRows<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.d.is_empty() {
            None
        } else {
            Some(self.d.drain(..self.n).collect())
        }
    }
}

impl<T> Iterator for IntoCols<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.rows[0].is_empty() {
            None
        } else {
            let num_rows = self.rows.len();

            let mut v = Vec::with_capacity(num_rows);
            for i in 0..num_rows {
                v.push(self.rows[i].pop_front().expect("Vecs must have the same length"));
            }

            Some(v)
        }
    }
}

impl<T> Iterator for IntoDiags<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.col_start == self.cols {
            return None
        }

        let mut v = Vec::with_capacity(self.diag_size);

        // The last_diag_elem always contain only one element.
        // We will use this element as a placeholder for no longer usable cells of the matrix.
        // SAFETY: last_diag_elem element is the last element popped out so it will continue to live till
        // the end of iteration.
        for _ in 0..self.diag_size  {
            v.push(unsafe {
                self.tmp_as_last_elem.read()
            })
        }

        let start = self.row_start * self.cols;
        let end = start + self.diag_size * self.cols;

        let stepby = self.cols + 1;

        let mut step = 0;
        let mut i = 0;

        while step < end - start {
            ::core::mem::swap(&mut self.d[start..end][step], &mut v[i]);
            i += 1;
            step += stepby;
        }

        let diags_with_same_size_on_axis = ::core::cmp::max(self.rows, self.cols) - ::core::cmp::min(self.rows, self.cols) + 1;

        if self.row_start == 0 {
            self.col_start += 1;

            if self.col_start >= diags_with_same_size_on_axis - 1 {
                self.diag_size -= 1;
            }
        }
        else {
            self.row_start -= 1;

            if self.row_start >= diags_with_same_size_on_axis - 1 {
                self.diag_size += 1;
            }
        }

        Some(v)
    }
}

impl<T> ExactSizeIterator for IntoRows<T> {
    fn len(&self) -> usize { self.n }
}
impl<T> ExactSizeIterator for IntoCols<T> {
    fn len(&self) -> usize {  self.rows.len()  }
}
impl<T> ExactSizeIterator for IntoDiags<T> {
    fn len(&self) -> usize { self.row_start }
}

impl<T> FusedIterator for IntoRows<T> {}
impl<T> FusedIterator for IntoCols<T> {}
impl<T> FusedIterator for IntoDiags<T> {}

impl<T> DoubleEndedIterator for IntoRows<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.d.is_empty() {
            None
        } else {
            let r = self.d.len() / self.n;
            Some(self.d.split_off(self.n * (r-1)).into())
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let r: usize = self.d.len() / self.n;

        if n >= r || self.d.is_empty() {
            None
        } else {
            let mut split = self.d.split_off(self.n * (r - n - 1));
            split.truncate(r);
            Some(split.into())
        }
    }
}
impl<T> DoubleEndedIterator for IntoCols<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rows[0].is_empty() {
            None
        } else {
            let num_rows = self.rows.len();

            let mut v = Vec::with_capacity(num_rows);
            for i in 0..num_rows {
                v.push(self.rows[i].pop_front().expect("Vecs must have the same length"));
            }

            Some(v)
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let row_len = self.rows[0].len();

        if n >= row_len || self.rows[0].is_empty() {
            None
        } else {
            let num_rows = self.rows.len();

            let mut v = Vec::with_capacity(num_rows);
            for i in 0..num_rows {
                let c = self.rows[i]
                    .split_off(row_len - n - 1)
                    .pop_front()
                    .unwrap();
                v.push(c);
            }
            Some(v)
        }
    }
}
