use crate::iterators::*;
use crate::access::*;

pub mod req;
pub mod ops;

pub use req::*;

/// This trait provides methods and tools for manipulating matrix-like structures.
pub trait Matrix
{
    /// The type of the elements of the matrix.
    type Element;
    
    
    // Required methods
    
    /// Gets the number of rows of the matrix.
    fn num_rows(&self) -> usize;

    /// Gets the number of columns of the matrix.
    fn num_cols(&self) -> usize;

    /// Returns a reference to an element inside the matrix, at the intersection of the `i`-th row and the `j`-th column.
    fn get(&self, row: usize, column: usize) -> Option<&Self::Element>;

    /// Returns a mutable reference to a value inside the matrix, at the intersection of the `i`-th row and the `j`-th column.
    fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut Self::Element>;

    
    
    // Provided methods.
    
    /// Returns a reference to an element, without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get(row_index, column_index).unwrap_unchecked()`.
    ///
    /// [`get`]: crate::traits::Matrix::get
    
    unsafe fn get_unchecked(&self, row: usize, column: usize) -> &Self::Element {
        self.get(row, column).unwrap_unchecked()
    }
    
    
    /// Returns a mutable reference to an element, without doing
    /// bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    ///
    /// You can think of this like `.get_mut(row_index, column_index).unwrap_unchecked()`.
    ///
    /// [`get_mut`]: crate::traits::Matrix::get_mut
    
    unsafe fn get_mut_unchecked(&mut self, row: usize, column: usize) -> &mut Self::Element {
        self.get_mut(row, column).unwrap_unchecked()
    }
    
    
    /// Returns the size of the matrix ie. `.num_rows()` * `.num_cols()`.
    fn size(&self) -> usize { self.num_rows() * self.num_cols() }
    
    /// Returns the dimensions of the matrix
    ///
    /// # Example
    /// ```rust
    /// use crate::matrixable::matrix;
    /// use crate::matrixable::prelude::Matrix;
    ///
    /// let m = matrix![[1, 1, 1], [2, 2, 2]];
    ///
    /// assert_eq!((2, 3), m.dimensions());
    /// ```
    fn dimensions(&self) -> (usize, usize) { (self.num_rows(), self.num_cols()) }

    /// Returns the number of diagonals.
    fn num_diags(&self) -> usize { self.num_cols() - 1 + self.num_rows() }

    /// Returns the length of a row.
    fn row_len(&self) -> usize { self.num_cols() }

    /// Returns the length of a column.
    fn col_len(&self) -> usize { self.num_rows() }


//     fn contains(&self, val: &Self::Element) -> bool 
//     where 
//         Self: Sized,
//         Self::Element: PartialEq 
//     {
//         self.iter().find(|&&x| x == val).is_some()
//     }
//     
//     fn check(&self, i: usize, j: usize) -> bool {
//         i < self.num_rows() && j < self.num_cols()
//     }
//     
    /// Helper method for converting subscripts of an eventual matrix element into an index in vector representation.
    fn index_from(&self, (i, j): (usize, usize)) -> usize {  
        i * self.num_cols() + j
    }

    /// Helper method for getting indexes from an index from vector representation.
    fn indexes_from(&self, n: usize) -> (usize, usize) { 
        (n / self.num_cols(), n % self.num_cols())
    }
    
    /// Checked index calculation.
    ///
    /// Returns None if indexes are out of bounds of the matrix.
    fn checked_index_from(&self, (i, j): (usize, usize)) -> Option<usize> {  
        let n = i * self.num_cols() + j;
        if n >= self.size() {
            None
        }
        else {
            Some(n)
        }
    }

    /// Checked indexes calculation.
    ///
    /// Returns None if index is out of bound of the vector representation.
    fn checked_indexes_from(&self, n: usize) -> Option<(usize, usize)> { 
        if n >= self.size() {
            None
        }
        else {
            Some((n / self.num_cols(), n % self.num_cols()))
        }
    }

    /// Changes the value of an element at the intersection of the `i`-th row and the `j`-th column of the matrix.
    ///
    /// # Error
    /// An error is returned if any of those indexes are out of bounds.
    fn set(&mut self, (i, j): (usize, usize), val: Self::Element) -> Result<(), &'static str> {
        match self.get_mut(i, j) {
            Some(target) => {
                *target = val;
                Ok(())
            }
            None => Err("Cannot access element from indexes."),
        }
    }    
    
    /// Returns an iterator over the elements of the matrix.
    ///
    /// Iteration follows the *Row Major Order*.
    fn iter(&self) -> Iter<'_, Self> where Self: Sized { Iter::new(self) }
    
    /// Returns an iterator that allows modifying each element.
    ///
    /// Iteration follows the *Row Major Order*.
    fn iter_mut(&mut self) -> IterMut<'_, Self> where Self: Sized { IterMut::new(self) }
    
    
    /// Returns an iterator over the elements of the `i`-th row.
    ///
    /// None is returned if `i >= number of rows`.
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

    /// Returns an iterator that allows modifying each element of the `i`-th row  .
    ///
    /// None is returned if `i >= number of rows`.
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

    /// Returns an iterator over elements of the `j`-th column.
    ///
    /// None is returned if `j >= number of columns`.
    fn column(&self, j: usize) -> Option<Column<'_, Self>> 
    where Self: Sized
    {
        if j >= self.num_cols() {
            None
        }
        else {
            Some(Column::new(self, j))
        }
    }

    /// Returns an iterator over that allows modifying each element of the `j`-th column.
    ///
    /// None is returned if `j >= number of columns`.
    fn column_mut(&mut self, j: usize) -> Option<ColumnMut<'_, Self>>
    where Self: Sized
    {
        if j >= self.num_cols() {
            None
        }
        else {
            Some(ColumnMut::new(self, j))
        }
    }   

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
    
    
    /// Returns an iterator over that allows modifying each element of the `n`-th diagonal.
    ///
    /// None is returned if `n >= number of diagonals`.
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
    
    
    /// Returns an iterator which gives the current subscripts of the current element as well as its value.
    fn enumerate(&self) -> Enumerator<Iter<'_, Self>>
    where Self: Sized
    {
        let cols = self.num_cols();
        Enumerator::new(self.iter(), cols)
    }

    /// `.enumerate()` with mutable access to each element.
    fn enumerate_mut(&mut self) -> Enumerator<IterMut<'_, Self>>
    where Self: Sized
    {
        let cols = self.num_cols();
        Enumerator::new(self.iter_mut(), cols)
    }


    /// Returns an iterator over the rows with immutable access to elements.
    fn rows(&self) -> Rows<Self> where Self: Sized { 
        Rows::from(self)
    }

    /// Returns an iterator over the rows with mutable access to elements.
    fn rows_mut(&mut self) -> RowsMut<Self> where Self: Sized {
        RowsMut::from(self) 
    }


    /// Returns an iterator over the columns with immutable access to elements.
    fn columns(&self) -> Columns<Self> where Self: Sized { 
        Columns::from(self)
    }

    /// Returns an iterator over the columns of the matrix with mutable access to elements.
    fn columns_mut (&mut self) -> ColumnsMut<Self> where Self: Sized {
        ColumnsMut::from(self) 
    }
    
    /// Returns an iterator over the diagonals with immutable access to elements.
    fn diagonals(&self) -> Diags<Self> where Self: Sized {
        Diags::from(self) 
    }

    /// Returns an iterator over the diagonals with mutable access to elements.
    fn diagonals_mut (&mut self) -> DiagsMut<Self> where Self: Sized {
        DiagsMut::from(self) 
    }

    /// Creates an iterator over rows from the matrix.
    fn into_rows(self) -> IntoRows<Self::Element> 
    where Self: Sized + Into<Vec<Self::Element>> { 
        IntoRows::from(self) 
    }

    /// Creates an iterator over columns from the matrix.
    fn into_cols(self) -> IntoCols<Self::Element>
    where Self: Sized + Into<Vec<Self::Element>> {
        IntoCols::from(self) 
    }

    /// Creates an iterator over diagonals from the matrix.
    fn into_diags(self) -> IntoDiags<Self::Element>
    where Self: Sized + Into<Vec<Self::Element>> {
        IntoDiags::from(self) 
    }

    /// Creates from matrix a matrix where access to elements is conditioned by `access`.
    fn into_access(self, access: fn(&Self, usize, usize)-> Option<(usize, usize)>) -> Access<Self>
    where Self: Sized {
        Access::new(self, access)
    }
    
    
    /// Wraps the matrix into one where access is defined by a matrix of indexes.
    fn into_access_map(self, mapping: crate::view::MatrixView<usize>) -> AccessMap<Self>  
    where Self: Sized {
        AccessMap { mapping, target: self } 
    }
    /*
    fn transform(self, t: TransformStrategy) -> Box<dyn Transformation>
    where Self: Sized + 'static  {
        match t {
            TransformStrategy::Identity => Box::new(Identity(self)),
            TransformStrategy::Transposition => Box::new(Transposition(self)),
            TransformStrategy::RotationR => Box::new(RotationR(self)),
            TransformStrategy::RotationL => Box::new(RotationL(self)),
            TransformStrategy::FlipH => Box::new(FlipH(self)),
            TransformStrategy::FlipV => Box::new(FlipV(self)),
            TransformStrategy::CentralSymmetry => Box::new(CentralSymmetry(self)),
            TransformStrategy::Map(mapping) => Box::new(Map { mapping, target: self })
        }
    }*/
     
//     /// Returns a submatrix
//     fn submatrix(self, edge1:(usize, usize), edge2: (usize, usize)) -> Self {
//         let row_len = edge1.1.abs_diff(egde.1) + 1;
//         let col_len = edge1.0.abs_diff(edge.0) + 1;
//         let skip = self.index_from(edge1);
//         self.enumerate()
//             .skip(skip)
//             .take_while(|(i, j, _)| (i, j) == edge2);
//         todo!()
//         
//     }
//   
/*    /// Performs an in-place transposition of the matrix.
    fn transposed(&mut self) -> &mut Self 
    where Self::Data: Swap<usize>
    {
        // element [0] and element[size-1] does not need to be transposed
        // so we reduce the array into all the elements between indices 0 and size-1
        // that is `1..=size-2`
        let limit = self.num_rows() * self.num_cols()  -  1;
        let r = self.num_rows();

        let mut hash = std::collections::HashSet::new();

        let mut dest: usize;
        for n in 1..limit {
            dest = (n * r) % limit;

            if hash.contains(&n) {
                continue;
            }

            hash.insert(dest);

            self.data_mut().swap(n, dest);  // Data again must implement swap
        }

        self.swap_dimensions();
        self
    }
*/

    /// Swaps to elements in the matrix.
    ///
    /// If a equals to b, it’s guaranteed that elements won’t change value.
    ///
    /// # Arguments
    ///
    /// a - The index of the first element
    /// b - The index of the second element
    ///
    /// # Panics
    ///
    /// Panics if a or b are out of bounds.
    fn swap(&mut self, a:(usize, usize), b:(usize, usize)) {
        if a == b { return }
        let a: *mut Self::Element = self.get_mut(a.0, a.1).unwrap();
        let b: *mut Self::Element = self.get_mut(b.0, b.1).unwrap();
        unsafe { std::mem::swap(&mut *a, &mut *b) };
    }

    /// After calling this function, `.num_rows()` becomes `.num_cols()` and vice-versa.
    fn swap_dimensions(&mut self) {
    
        println!("Before permutation: rows {}, cols {}", self.num_rows(), self.num_cols());
        /*let mut a: *mut fn(&Self)->usize = Self::num_rows;
        let mut b: *mut fn(&Self)->usize = Self::num_cols;
        */
        let a = <self as Matrix>::num_rows as usize;
        let b = <self as Matrix>::num_cols as usize;
        println!("addr fnrow {a},, addr fncol{b}");
        let c = 1;
        unsafe {std::mem::swap(&mut *(a as *mut fn(&Self)->usize), &mut *(b as *mut fn(&Self)->usize))};
        
        println!("After permutation: rows {}, cols {}",  self.num_rows(), self.num_cols());
    }
    
    /// Checks if the matrix is a square matrix (a matrix with equal number of rows and columns).
    fn is_square(&self) -> bool {
        self.num_rows() == self.num_cols()
    }

    /// Checks if the matrix has one dimension (number of columns is `1` or number of rows is `1`)
    fn is_one_dimension(&self) -> bool {
        self.num_rows() == 1 || self.num_cols() == 1
    }

    /// Checks if the matrix is symmetric i.e. it does not change when transposed
    fn is_symmetric(&self) -> bool
    where
        Self::Element: PartialEq
    {
        let r = self.num_rows();
        let c = self.num_cols();
        let limit = r * c  -  1;
        let mut hash = std::collections::HashSet::new();

        let mut dest: usize;
        for n in 1..limit {
            dest = (n * r) % limit;

            if hash.contains(&n) {
                continue;
            }

            hash.insert(dest);

            let (i, j) = crate::indexes_from(c, n);
            let (i_dest, j_dest) = crate::indexes_from(c, dest);
            
            if self.get(i, j).unwrap() != self.get(i_dest, j_dest).unwrap() {
                return false
            }
        }

        true
    }

    /// Checks if the matrix is a singleton i.e. dimensions are equal to (1x1).
    fn is_singleton(&self) -> bool {
        self.dimensions() == (1, 1)
    }
    
    /// Checks if the matrix is horizontal (number of rows of the matrix is lower than number of columns).
    fn is_horizontal(&self) -> bool {
        self.num_rows() <= self.num_cols()
    }
    
    
    /// Checks if the matrix is vertical (number of rows of the matrix is greater than number of columns).
    fn is_vertical(&self) -> bool {
        self.num_rows() >= self.num_cols()
    }
    
    /// Returns a boolean indicating if the matrix looks like a diagonal matrix (a matrix which entries outside the main diagonal are all zero), along with the reference to the element that may serve as zero in that matrix if the check was correct.
    
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

    
    /// Returns a boolean indicating if matrix is a ~square diagonal matrix~ having the same elements on its diagonal, along with that element and the element considered as zero.
    ///
    /// This method can be useful for finding an identity matrix.
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
        
        let one = self.get(0, 0).expect("First element.");
        
        // index (0, 1) or (1, 0): not on the main diagonal and must be the same value everywhere except on that diagonal
        let zero= if let Some(z) = self.get(0, 1) {
            z 
        } else {
            self.get(1, 0).expect("Second element either from row or column")
        };
        
        if one == zero { 
            return (false, None, None)
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
}

