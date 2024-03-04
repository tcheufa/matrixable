//! This module contains structs for iterating over matrices.
//!
//! These structs also implement [`Index`](std::ops::Index) allowing you to use the `container[index]` notation.
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
//! It may happen you just need a unique element. In that case you will rather use [`.nth()`](std::iter::Iterator::nth) 
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
use std::{
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
};

/// TODO
/// - Add a feature for itertools crate (or even not finally)
/// - Add a feature for serde crate for serialization of matrixlikes (or even not finally)

//use crate::view::MatrixExtView;
use crate::{MatrixExt, MatrixMutExt};

macro_rules! iter {
    (
        $($name:ident { $($mut:ident)? } { $ptr:ident } $matrixTrait:ident $getfn:ident $($start:ident)?),* ;
        $get_ij:expr ;
        $incrfn:item ;
        $lenimpl:item ;
        $nextbackimpl:item 
    ) => {
        $(
            #[derive(Hash, Debug)]
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
                F: std::ops::Deref<Target = [T]>,
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
                F: std::ops::Deref<Target = [T]>,
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
            
            impl<'a, M: $matrixTrait> std::ops::Index<usize> for $name<'a, M>
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
                impl<'a, M: $matrixTrait> std::ops::IndexMut<usize> for $name<'a, M>
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


iter!{Iter {/*no mut */} { const } MatrixExt get, IterMut { mut } { mut } MatrixMutExt get_mut;
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

iter!{Row {/*no mut */} { const } MatrixExt get row, RowMut { mut } { mut } MatrixMutExt get_mut row;
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

iter!{Column {/*no mut */} { const } MatrixExt get col, ColumnMut { mut } { mut } MatrixMutExt get_mut col;
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

iter!{Diag {/*no mut */} { const } MatrixExt get n, DiagMut { mut } { mut } MatrixMutExt get_mut n;
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


impl<'a, M: MatrixExt> Copy for Iter<'a, M>
where M::Element: 'a {}

impl<'a, M: MatrixExt> Copy for Row<'a, M>
where M::Element: 'a {}

impl<'a, M: MatrixExt> Copy for Column<'a, M>
where M::Element: 'a {}

impl<'a, M: MatrixExt> Copy for Diag<'a, M>
where M::Element: 'a {}



impl<'a, M: MatrixExt> Clone for Iter<'a, M>
where M::Element: 'a {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, M: MatrixExt> Clone for Row<'a, M>
where M::Element: 'a {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, M: MatrixExt> Clone for Column<'a, M>
where M::Element: 'a {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, M: MatrixExt> Clone for Diag<'a, M>
where M::Element: 'a {
    fn clone(&self) -> Self {
        *self
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
            self.i += 1;
        }
        
        Some(next_back)  
    }
}
impl<I: FusedIterator> FusedIterator for Enumerator<I> {}
