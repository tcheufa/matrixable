use std::{
    fmt::Debug,
    iter::FusedIterator,
    marker::PhantomData,
    collections::VecDeque,
};

/// TODO
/// - Add a feature for itertools crate (or even not finally)
/// - Add a feature for serde crate for serialization of matrixlikes (or even not finally)
/// - Implement DoubleEndedIterator and ExactSizeIterator for Diag and IntoDiags

//use crate::view::MatrixView;
use crate::traits::Matrix;

#[derive(Clone, Copy, Debug, Default)]
/// Choose which mode for extending a matrix.
pub enum Extend {
    /// Extend by row.
    #[default] Row,
    /// Extend by column.
    _Column
}


macro_rules! iter {
    (
        $($name:ident { $($mut:ident)? } { $ptr:ident } $getfn:ident $($start:ident)?),* ;
        $get_ij:expr ;
        $incrfn:item ;
        $lenimpl:item ;
        $nextbackimpl:item 
    ) => {
        $(
            #[derive(Debug)]
            pub struct $name<'a, M: Matrix>
            where M::Element: 'a 
            {
                m: *$ptr M,
                i: usize,
                j: usize,
                _marker: PhantomData<&'a M>,
            }
        
            impl<'a, M: Matrix> $name<'a, M>
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
            
            impl<'a, M: Matrix> Iterator for $name<'a, M> 
            where <M as Matrix>::Element: 'a 
            {
                type Item = &'a $($mut)? <M as Matrix>::Element;
                    fn next(&mut self) -> Option<Self::Item> {
                    let (i, j) = (self.i, self.j);
                    let next = self.get(i, j)?;
                    self.increment();
                    
                    // SAFETY: Nothing else points to or will point to the contents of this iterator.
                    Some(next)
                }
            }
            
            impl<'a, M: Matrix> FusedIterator for $name<'a, M> {}

            impl<'a, M: Matrix> ExactSizeIterator for $name<'a, M> {
                $lenimpl
            }

            impl<'a, M: Matrix> DoubleEndedIterator for $name<'a, M> {
                $nextbackimpl
            }


            impl<'a, M: Matrix, T> FromIterator<$name<'a, M>> for Vec<Vec<T>>
            where
                Vec<T>: FromIterator<<$name<'a, M> as Iterator>::Item>,
                M::Element: 'a
            {
                fn from_iter<IntoI>(iter: IntoI) -> Self
                where
                    IntoI: IntoIterator<Item = $name<'a, M>>
                {
                    iter.into_iter().map(|c| c.collect::<Vec<_>>()).collect()
                }
            }
        )*
    }
}



iter!{Iter {/*no mut */} { const } get, IterMut { mut } { mut } get_mut;
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

iter!{Row {/*no mut */} { const } get row, RowMut { mut } { mut } get_mut row;
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

iter!{Column {/*no mut */} { const } get col, ColumnMut { mut } { mut } get_mut col;
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

iter!{Diag {/*no mut */} { const } get n, DiagMut { mut } { mut } get_mut n;
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
        $outElem:ty,
        $callfn:ident,
        $lenfn:ident
    ) => {
    
        #[derive(Clone, Debug)]
        pub struct $w<'a, M>
        where M: Matrix,
        <M as Matrix>::Element: 'a {
            n: usize,
            m:  *$ptr M,
            _marker: PhantomData<&'a M>
        }

        impl<'a, M> Iterator for $w<'a, M> 
        where
            M: Matrix,
            <M as Matrix>::Element: 'a
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
        where M: Matrix,
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
        where M: Matrix
        {  }
        
        impl<'a, M> ExactSizeIterator for $w<'a, M>
        where M: Matrix {
            fn len(&self) -> usize {
                let m = unsafe { &$($mut)? *self.m };
                 m.$lenfn()
            }
        }

        impl<'a, 'b, M> From<&'b $($mut)? M> for $w<'a, M>
        where 
            'b: 'a,
            M: Matrix
        {
            fn from(source: &'b $($mut)? M) -> Self { Self { n:0, m: source, _marker: PhantomData } }
        }
    };
}


dimensional_iterator!{ 
    Rows, const, { /* no mut */},
    Row<'a, M>,
    row, num_rows 
}

dimensional_iterator!{ 
    RowsMut, mut, { mut },
    RowMut<'a, M>,
    row_mut, num_rows 
}

dimensional_iterator!{ 
    Columns, const, {/* no mut */},
    Column<'a, M>,
    column, num_cols
}

dimensional_iterator!{ 
    ColumnsMut, mut, { mut },
    ColumnMut<'a, M>,
    column_mut, num_cols 
}

dimensional_iterator!{ 
    Diags, const, { /* no mut */ },
    Diag<'a, M>,
    diag, num_diags 
}

dimensional_iterator!{ 
    DiagsMut, mut, { mut },
    DiagMut<'a, M>,
    diag_mut, num_diags 
}

pub struct IntoRows<T> {
    d: Vec<T>,
    n: usize
}
pub struct IntoCols<T> {
    rows: Box<[VecDeque<T>]>,
}
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


impl<M> From<M> for IntoRows<<M as Matrix>::Element>
where M: Matrix + Into<Vec<M::Element>>
{
    fn from(value: M) -> Self {
        Self { n: value.num_cols(), d: value.into() }
    }
}
impl<M> From<M> for IntoCols<<M as Matrix>::Element>
where M: Matrix + Into<Vec<M::Element>> 
{
    fn from(value: M) -> Self {
        let (rows, cols) = (value.num_rows(), value.num_cols());
        let mut v = Vec::with_capacity(rows);
        let mut into_vec: Vec<_> = value.into();
        for _ in 0..rows {
            v.push(into_vec.drain(..cols).collect())
        }

        Self { rows: v.into() }
    }
}
impl<M> From<M> for IntoDiags<<M as Matrix>::Element> 
where M : Matrix + Into<Vec<M::Element>>
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
            d: value.into(),
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
        // We will this element as a placeholder for no longer usable cells of the matrix.
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
            std::mem::swap(&mut self.d[start..end][step], &mut v[i]);
            i += 1;
            step += stepby;
        }

        let diags_with_same_size_on_axis = std::cmp::max(self.rows, self.cols) - std::cmp::min(self.rows, self.cols) + 1;

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


// macro_rules! new_iterator_wrapper {
//     ($w:ident < $I:ident > = $iter:ty) => {
//         #[derive(Debug)]
//         pub struct $w<$I> ($iter) where $I: Iterator;
// 
//         impl<$I: Iterator> Iterator for $w<$I>{
//             type Item = <$iter as Iterator>::Item;
// 
//             fn next(&mut self) -> Option<Self::Item> { self.0.next() }
//         }
// 
//         impl<$I: FusedIterator> FusedIterator for $w<$I> {}
// 
//         impl<$I: Iterator> ExactSizeIterator for $w<$I> 
//         where $iter: ExactSizeIterator
//         {
//             fn len(&self) -> usize { self.0.len()  }
//         }
// 
//         impl<$I: Iterator> DoubleEndedIterator for $w<$I> 
//         where $iter: DoubleEndedIterator
//         {
//             fn next_back(&mut self) -> Option<Self::Item> { self.0.next_back() }
//         }
// 
//         impl<$I: Iterator> From<$iter> for $w<$I> {
//             fn from(source: $iter) -> Self { Self(source) }
//         }
//         
//         
//         impl<$I: Iterator, T> FromIterator<$w<$I>> for Vec<Vec<T>>
//         where
//             Vec<T>: FromIterator<<I as Iterator>::Item>
//         {
//             fn from_iter<IntoI>(iter: IntoI) -> Self
//             where
//                 IntoI: IntoIterator<Item = $w<$I>>
//             {
//                 iter.into_iter().map(|c| c.collect::<Vec<_>>()).collect()
//             }
//         }
//     };
// }
// 

/*
new_iterator_wrapper! { RowIter<I> = Take<Skip<I>> }

new_iterator_wrapper! { RowIterMut<I> = Take<Skip<I>> }

new_iterator_wrapper! { ColumnIter<I> = StepBy<Skip<I>> }

new_iterator_wrapper! { ColumnIterMut<I> = StepBy<Skip<I>> }

new_iterator_wrapper! { DiagIter<I> = Take<StepBy<Skip<I>>> }

new_iterator_wrapper! { DiagIterMut<I> = Take<StepBy<Skip<I>>> }*/
