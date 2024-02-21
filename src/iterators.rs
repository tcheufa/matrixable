use std::{
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
};

/// TODO
/// - Add a feature for itertools crate (or even not finally)
/// - Add a feature for serde crate for serialization of matrixlikes (or even not finally)
/// - Implement DoubleEndedIterator and ExactSizeIterator for Diag

//use crate::view::MatrixView;
use crate::traits::{Matrix, MatrixMut};

macro_rules! iter {
    (
        $($name:ident { $($mut:ident)? } { $ptr:ident } $matrixTrait:ident $getfn:ident $($start:ident)?),* ;
        $get_ij:expr ;
        $incrfn:item ;
        $lenimpl:item ;
        $nextbackimpl:item 
    ) => {
        $(
            #[derive(Debug)]
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
                
                #[inline]
                fn use_matrix(&$($mut)? self) -> &'a $($mut)? M {
                    unsafe { (&$($mut)? *self.m) }
                } 
                
                #[inline]
                fn get(&$($mut)? self, i: usize, j: usize) -> Option<&'a $($mut)? M::Element> {
                    self.use_matrix().$getfn(i, j)
                }
                
                #[inline]
                $incrfn
                
            }
            
            impl<'a, M: $matrixTrait> Iterator for $name<'a, M> 
            where M::Element: 'a 
            {
                type Item = &'a $($mut)? M::Element;
                    fn next(&mut self) -> Option<Self::Item> {
                    let (i, j) = (self.i, self.j);
                    let next = self.get(i, j)?;
                    self.increment();
                    
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
        )*
    }
}



iter!{Iter {/*no mut */} { const } Matrix get, IterMut { mut } { mut } MatrixMut get_mut;
    |_m: &M| (0, 0) ;
    fn increment(&mut self) {
        self.j += 1;
        if self.j == self.use_matrix().num_cols() {
                self.j = 0;
                self.i += 1;
        }
    } ;
    fn len(&self) -> usize {  unsafe {&*self.m}.size()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let (rows, cols) = m.dimensions();
        let (i, j) = (rows - self.i - 1, cols - self.j - 1);
        
        self.increment();
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get(i, j)
    }
}

iter!{Row {/*no mut */} { const } Matrix get row, RowMut { mut } { mut } MatrixMut get_mut row;
    |_m: &M, row| (row, 0) ;
    fn increment(&mut self) {  
        self.j += 1;
    } ;
    fn len(&self) -> usize {  unsafe {&*self.m}.row_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let cols = m.num_cols();
        let (i, j) = (self.i, cols - self.j - 1);
        let next = self.get(i, j)?;
        
        self.increment();
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        Some(next)
    }
}

iter!{Column {/*no mut */} { const } Matrix get col, ColumnMut { mut } { mut } MatrixMut get_mut col;
    |_m: &M, col| (0, col) ;
    fn increment(&mut self) {  
        self.i += 1;
    } ; 
    fn len(&self) -> usize { unsafe {&*self.m}.col_len()  } ;
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let rows = m.num_rows();
        let (i, j) = (rows - self.i - 1, self.j);
        
        self.increment();
        
        // SAFETY: Nothing else points to or will point to the contents of this iterator.
        self.get(i, j)
    }
}

iter!{Diag {/*no mut */} { const } Matrix get n, DiagMut { mut } { mut } MatrixMut get_mut n;
    |m: &M, mut n| {
        let lastrow = m.num_rows() - 1;
        if n <= lastrow {
            (lastrow - n, 0) 
        } else {
            n = n - lastrow;
            (0, n)
        }
    };
    fn increment(&mut self) {  
        self.i += 1;
        self.j += 1;
    }; 
    fn len(&self) -> usize { 
        //let m = unsafe {&*self.m};
        //m.index_from((m.num_rows() - i, m.num_cols()))
        0
    };
    fn next_back(&mut self) -> Option<Self::Item> {
        let m = self.use_matrix();
        let (rows, cols) = m.dimensions();
        let (i, j) = (rows - self.i - 1, cols - self.j - 1);
        
        self.increment();
        
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
    
        #[derive(Clone, Debug)]
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
            fn from(source: &'b $($mut)? M) -> Self { Self { n:0, m: source, _marker: PhantomData } }
        }
    };
}


dimensional_iterator!{ 
    Rows, const, { /* no mut */}, Matrix,
    Row<'a, M>,
    row, num_rows 
}

dimensional_iterator!{ 
    RowsMut, mut, { mut }, MatrixMut,
    RowMut<'a, M>,
    row_mut, num_rows 
}

dimensional_iterator!{ 
    Columns, const, {/* no mut */}, Matrix,
    Column<'a, M>,
    column, num_cols
}

dimensional_iterator!{ 
    ColumnsMut, mut, { mut }, MatrixMut,
    ColumnMut<'a, M>,
    column_mut, num_cols 
}

dimensional_iterator!{ 
    Diags, const, { /* no mut */ }, Matrix,
    Diag<'a, M>,
    diag, num_diags 
}

dimensional_iterator!{ 
    DiagsMut, mut, { mut }, MatrixMut,
    DiagMut<'a, M>,
    diag_mut, num_diags 
}


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
