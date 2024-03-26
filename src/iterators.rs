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
    iter::{ FusedIterator, FromIterator },
    marker::PhantomData,
    
};


use alloc::vec::Vec;
use alloc::vec::IntoIter;
use alloc::boxed::Box;

use crate::{MatrixExt, MatrixMutExt};

macro_rules! iter {
    (
        $(#[doc = $doc:expr] $name:ident { $($mut:ident)? } { $ptr:ident } $matrixTrait:ident $getfn:ident $($start:ident)?),* ;
        $get_bounds:expr ;
        $incrfn:item ;
        $lenimpl:item ;
        $nextbackimpl:item 
    ) => {
        $(
            #[doc = $doc]
            /// See its documentation for more.
            #[derive(Hash, Debug, Copy, Clone)]
            pub struct $name<'a, M: $matrixTrait + 'a> 
            {
                m: *$ptr M,
                i: usize,
                irev: usize,
                _marker: PhantomData<&'a M>,
            }
        
            impl<'a, M: $matrixTrait> $name<'a, M>
            where M::Element: 'a 
            {
                pub(crate) fn new(m: &'a $($mut)? M $(, $start: usize)? ) -> Self {
                    let get_start = $get_bounds;
                    let (i, irev) = get_start(m $(, $start)?);
                    Self { m, i, irev, _marker: PhantomData }
                }
                
                #[inline(always)]
                fn use_matrix(&$($mut)? self) -> &'a $($mut)? M {
                    unsafe { (&$($mut)? *self.m) }
                }         
                
                #[inline(always)]
                fn matrix(&self) -> &'a M {
                    unsafe { (&*self.m) }
                } 
                
                #[inline(always)]
                fn get_nth(&$($mut)? self, i: usize) -> Option<&'a $($mut)? M::Element> {
                    self.use_matrix().$getfn(i)
                }
                
                #[inline(always)]
                $incrfn
                
            }
            
            impl<'a, M: $matrixTrait> Iterator for $name<'a, M> 
            where M::Element: 'a 
            {
                type Item = &'a $($mut)? M::Element;
                    fn next(&mut self) -> Option<Self::Item> {
                    if self.i > self.irev {
                        return None
                    }
                    let i = self.i;    
                    self.i = self.increment(i);

                    // SAFETY: Nothing else points to or will point to the contents of this iterator.
                    self.get_nth(i)
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
                    let mut i = self.i;
                    for _ in 0..idx {
                        i = self.increment(i);
                    }
                    self.matrix().get_nth(i).unwrap()
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
                        let mut i = self.i;
                        for _ in 0..idx {
                            i = self.increment(i);
                        }
                        self.get_nth(i).unwrap()
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
    Iter {/*no mut */} { const } MatrixExt get_nth,
    #[doc = 
    "An iterator over the elements of the matrix (mutable).\n\n\
    This struct is created by the [`iter_mut`](MatrixMutExt::iter_mut) method on [`MatrixMutExt`]."]
    IterMut { mut } { mut } MatrixMutExt get_nth_mut;
    |m: &M| (0, m.size().saturating_sub(1)) ;
    fn increment(&self, i: usize) -> usize {
        i + 1
    } ;
    fn len(&self) -> usize { self.matrix().size()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i > self.irev {
            return None
        }
        let j = self.irev;
        self.irev -= 1;
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get_nth(j)
    }
}

iter!{
    #[doc = 
    "An iterator over a matrix row.\n\n\
    This struct is created by the [`row`](MatrixExt::row) method on [`MatrixExt`]."]
    Row {/*no mut */} { const } MatrixExt get_nth irow,
    #[doc = 
    "An iterator over a mutable matrix row.\n\n\
    This struct is created by the [`row_mut`](MatrixMutExt::row_mut) method on [`MatrixMutExt`]."]
    RowMut { mut } { mut } MatrixMutExt get_nth_mut irow;
    |m: &M, row| {
        let rlen = m.row_len();
        let i = row * rlen;
        (i, i + rlen - 1)
    } ;
    fn increment(&self, i: usize) -> usize {  
        i + 1
    } ;
    fn len(&self) -> usize {  self.matrix().row_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i > self.irev {
            return None
        }
        let j = self.irev;
        self.irev -= 1;
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get_nth(j)
    }
}

iter!{
    #[doc = 
    "An iterator over a matrix column.\n\n\
    This struct is created by the [`col`](MatrixExt::col) method on [`MatrixExt`]."]
    Column {/*no mut */} { const } MatrixExt get_nth icol,
    #[doc = 
    "An iterator over a mutable matrix column.\n\n\
    This struct is created by the [`col_mut`](MatrixMutExt::col_mut) method on [`MatrixMutExt`]."]
    ColumnMut { mut } { mut} MatrixMutExt get_nth_mut icol;
    |m: &M, col| {
        let (rows, cols) = m.dimensions();
        (col, (rows * cols) - cols.saturating_sub(col))
    } ;
    fn increment(&self, i: usize) -> usize {  
        i + self.matrix().row_len()
    } ; 
    fn len(&self) -> usize { self.matrix().col_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i > self.irev {
            return None
        }
        let j = self.irev;
        self.irev -= self.use_matrix().row_len();
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get_nth(j)
    }
}

iter!{
    #[doc =
    "An iterator over a matrix diagonal.\n\n\
    This struct is created by the [`diag`](MatrixExt::diag) method on [`MatrixExt`]."]
    Diag {/*no mut */} { const } MatrixExt get_nth n,
    #[doc =
    "An iterator over a mutable matrix diagonal.\n\n\
    This struct is created by the [`diag_mut`](MatrixMutExt::diag_mut) method on [`MatrixMutExt`]."]
    DiagMut { mut } { mut } MatrixMutExt get_nth_mut n;
    |m: &M, mut n| {
        let (rows, cols) = match m.dimensions() {
            (_, 0) | (0, _) => return (0, 1),
            (rows, cols) => (rows, cols)
        } ;
        let diag_len = m.diag_len(n);
        let main_diag = rows - 1;
        if n < main_diag {
            n = main_diag - n;
            (
                n * cols,
                n + cols * (diag_len + 1), 
            )
        } else {
            n = n - main_diag;
            (
                n,
                n + cols * (diag_len + 1), 
            )
        }
    } ;
    fn increment(&self, i: usize) -> usize {
        let m = self.matrix();
        let (mut i, mut j) = m.subscripts_from(i);
        i += 1;
        j += 1;
        
        if m.check(i, j) {
            m.index_from((i, j))
        }
        else {
            // Stop a further call to `next` method by passing value that ends iteration
            //(iteration goes until self.i > self.irev).
            self.irev + 1
        }
    }; 
    fn len(&self) -> usize {  self.matrix().diag_len(self.i) };
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i > self.irev {
            return None
        }
        let j = self.irev;
        self.irev -= self.use_matrix().row_len() + 1; 
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get_nth(j)
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
impl<I: FusedIterator> FusedIterator for Enumerator<I> {}


#[derive(Default, Clone, Debug)]
pub struct IntoAxes<T> {
    axes: IntoIter<Vec<T>>,
}
impl<T> IntoAxes<T>
{
    pub fn from_as_rows<M: MatrixExt>(src: M) -> Self
    where M: IntoIterator,
    <M as IntoIterator>::Item: IntoIterator<Item = T>
    {
        let mut v = Vec::with_capacity(src.num_rows());
        for row in src {
            v.push(row.into_iter().collect());
        }

        Self { axes: v.into_iter() }
    }

    pub fn from_as_cols<M: MatrixExt>(src: M) -> Self
    where M: IntoIterator,
    <M as IntoIterator>::Item: IntoIterator<Item = T>
    {
        let (num_rows, num_cols) = src.dimensions();
        let mut v = Vec::with_capacity(num_cols);
        for _ in 0..num_cols {
            v.push(Vec::with_capacity(num_rows));
        }
        for row in src {
            for (i, elem) in row.into_iter().enumerate() {
                v[i].push(elem)
            }
        }
        Self { axes: v.into_iter() }
    }

    pub fn from_as_diags<M: MatrixExt>(src: M) -> Self
        where M: IntoIterator,
              <M as IntoIterator>::Item: IntoIterator<Item = T>
    {
        if src.is_empty () {
            return Self {
                axes: Vec::new().into_iter()
            }
        }

        //let (num_rows, num_cols) = (value.num_rows(), value.num_cols());
        let num_diags = src.num_diags();
        let i_main_diag = src.num_rows() - 1;

        let mut v = Vec::with_capacity(num_diags);
        for i in 0..num_diags {
            v.push(Vec::with_capacity(src.diag_len(i)));
        }

        //let (mut i, mut j) = (rows - 1, 0);
        let mut into_iter = src.into_iter();

        let first_row = into_iter.next().expect("Not empty so first row must exist");

        // upper diagonals start
        for (i, elem) in first_row.into_iter().enumerate() {
            v[i_main_diag + i].push(elem);
        }

        // now continue iteration till the bottom
        for (i, row) in into_iter.enumerate() {
            let mut row = row.into_iter();

            // a new lower diagonal at each row
            // first element of that diag is first element of row
            v[i_main_diag - i - 1].push(row.next().expect("If new row then must contain at least one element"));

            for (j, elem) in row.enumerate() {
                v[i_main_diag + j].push(elem);
            }
        }
        Self { axes: v.into_iter() }
    }
}

impl<T> Iterator for IntoAxes<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.axes.next()
    }
}

impl<T> DoubleEndedIterator for IntoAxes<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.axes.next_back()
    }
}

impl<T> ExactSizeIterator for IntoAxes<T> {
    fn len(&self) -> usize { self.axes.len() }
}

impl<T> FusedIterator for IntoAxes<T> {}
