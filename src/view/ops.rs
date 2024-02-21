//! Adds operation capabilities to the `MatrixView`.
//!
//! Those operations are:
//! - `Add`
//! - `AddAssign`
//! - `Sub`
//! - `SubAssign`
//! - `Mul`
//! - `MulAssign`
//! - `PartialEq`
//!
//! The implementation of those traits has been done following the classical behavior of matrices in linear algebra.

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::view::MatrixView;
use crate::traits::Matrix;

/// With this implementation row vector and a column vector can be equal if their elements are equal.
impl<U, T> PartialEq<MatrixView<U>> for MatrixView<T>
where T: PartialEq<U> {
    fn eq(&self, other: &MatrixView<U>) -> bool {
        self.d == other.d
        &&
        (
            (self.num_rows() == other.num_rows() && self.c == other.c)
            ||
            (self.is_one_dimension() && other.is_one_dimension())
        )
    }
}

/// # Panics
/// A panic will occur if `lhs.num_cols() != rhs.num_cols()` or `lhs.num_rows() != rhs.num_rows()`.
impl<U, T> Add<MatrixView<U>> for MatrixView<T>
where
    T: Add<U>,
{
    type Output = MatrixView<<T as Add<U>>::Output>;

    fn add(self, other: MatrixView<U>) -> Self::Output {
        let self_r = self.num_rows();
        if self.num_cols() != other.num_cols() || self_r != other.num_rows() {
            panic!("Number of columns or rows of matrixes do not match")
        }

        let mut data = Vec::with_capacity(self.size());

        for (lhr, rhs) in self.d.into_iter().zip(other.d.into_iter()) {
            data.push(lhr + rhs);
        }

        MatrixView::new(data, self.c, self_r)
    }
}

/// # Panics
/// A panic will occur if `lhs.num_cols() != rhs.num_cols()` or `lhs.num_rows() != rhs.num_rows()`.
impl<U, T> Sub<MatrixView<U>> for MatrixView<T>
where
    T: Sub<U>,
{
    type Output = MatrixView<<T as Sub<U>>::Output>;

    fn sub(self, other: MatrixView<U>) -> Self::Output {
        let self_r = self.num_rows();
        if self.num_cols() != other.num_cols() || self_r != other.num_rows() {
            panic!("Number of columns or rows of matrixes do not match")
        }

        let mut data = Vec::with_capacity(self.size());

        for (lhr, rhs) in self.d.into_iter().zip(other.d.into_iter()) {
            data.push(lhr - rhs);
        }

        MatrixView::new(data, self.c, self_r)
    }
}

/// # Panics
/// A panic will occur if:
/// - The two matrices both have 1 dimension but their sizes are not equal.
/// - One of the two matrices has more than 1 dimension but `lhs.num_cols() != rhs.num_rows()`.
impl<U: Copy, T: Copy, O> Mul<MatrixView<U>> for MatrixView<T>
where
    T: Mul<U, Output = O>,
    O: Add<O, Output = O>
{
    type Output = MatrixView<O>;

    fn mul(self, rhs: MatrixView<U>) -> Self::Output {
        if self.is_one_dimension() && rhs.is_one_dimension() {
            if self.size() == rhs.size() {
                let mut d: Vec<O> = Vec::with_capacity(self.size());
                
                let self_r = self.num_rows();

                for (lhs, rhs) in self.d.into_iter().zip(rhs.d.into_iter()) {
                    d.push(lhs * rhs);
                }
                
                return MatrixView::new(d, self_r, self.c);
            }
            else {
                panic!("lhs size and rhs size must be equal for one dimension matrices")
            }
        }

        if self.num_cols() != rhs.num_rows() {
            panic!("lhs number of columns is not equal to rhs number of rows");
        }

        let (r, c) = (self.num_rows(), rhs.num_cols());

        let mut data = Vec::with_capacity(r * c);

        // TODO: Make this operation blazing fast
        for i in 0..self.num_rows() {
            for col in rhs.columns() {
                data.push(self.row(i).unwrap()
                          .zip(col)
                          .map(|(&i, &j)| i * j)
                          .reduce(|acc, product| acc + product)
                          .expect("Iterator not empty"));
            }
        }

        MatrixView::new(data, r, c)
    }
}

/// # Panics
/// - One of the two matrices has more than 1 dimension but `lhs.num_cols() != rhs.num_rows()`.
impl<U, T> AddAssign<MatrixView<U>> for MatrixView<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, other: MatrixView<U>) {
        self.d.iter_mut()
              .zip(other.d)
              .for_each(|(t, rhs)| *t += rhs);
    }
}


/// # Panics
/// - One of the two matrices has more than 1 dimension but `lhs.num_cols() != rhs.num_rows()`.
impl<U, T> SubAssign<MatrixView<U>> for MatrixView<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, other: MatrixView<U>) {
        self.d.iter_mut()
              .zip(other.d)
              .for_each(|(t, rhs)| *t -= rhs);
    }
}

/// # Panics
/// A panic will occur if:
/// - The two matrices both have 1 dimension but their sizes are not equal.
/// - One of the two matrices has more than 1 dimension but `lhs.num_cols() != rhs.num_rows()`.
impl<U: Copy, T: Copy> MulAssign<MatrixView<U>> for MatrixView<T>
where
    T: Add<T, Output = T> + Mul<U, Output = T>,
{
    fn mul_assign(&mut self, rhs: MatrixView<U>) {
        *self = self.clone() * rhs;
    }
}

macro_rules! multi_impl_commutative_mul_and_assign_for {
    ($($ty:ty)*) => {
        $(
            impl<T: Copy> Mul<MatrixView<T>> for $ty
            where
                $ty: Mul<T>
            {
                type Output = MatrixView<<$ty as Mul<T>>::Output>;

                fn mul(self, rhs: MatrixView<T>) -> Self::Output {
                    let mut d = Vec::with_capacity(rhs.size());
                    let rhs_r = rhs.num_rows();
                    for x in rhs.d {
                        d.push(self * x);
                    }

                    MatrixView::new(d, rhs_r, rhs.c)
                }
            }

            impl<'a, T: Copy> Mul<&MatrixView<T>> for &'a $ty
            where
                &'a $ty: Mul<T>
            {
                type Output = MatrixView<<&'a $ty as Mul<T>>::Output>;

                fn mul(self, rhs: &MatrixView<T>) -> Self::Output {
                    let mut d = Vec::with_capacity(rhs.size());
                    for x in rhs.d.iter() {
                        d.push(self * *x);
                    }

                    MatrixView::new(d, rhs.num_rows(), rhs.c)
                }
            }
        )*
    };
}


multi_impl_commutative_mul_and_assign_for!{ u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }


impl<U: Copy, T> Mul<U> for MatrixView<T>
where
    T: Mul<U>
{
    type Output = MatrixView<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut d: Vec<<T as Mul<U>>::Output> = Vec::with_capacity(self.size());
        let self_r = self.num_rows();
        for x in self.d {
            d.push(x * rhs);
        }

        MatrixView::new(d, self_r, self.c)
    }
}


impl<'a, 'b, U: Copy, T> Mul<&'a U> for &'b MatrixView<T>
where
    &'b T: Mul<&'a U>,
{
    type Output = MatrixView<<&'b T as Mul<&'a U>>::Output>;

    fn mul(self, rhs: &'a U) -> Self::Output {

        let mut d = Vec::with_capacity(self.size());
        for x in &self.d {
            d.push(x * rhs);
        }

        MatrixView::new(d, self.num_rows(), self.c)
    }
}

impl<U: Copy, T: Copy> MulAssign<U> for MatrixView<T>
where
    U: Mul<MatrixView<T>, Output = MatrixView<T>>
{
    fn mul_assign(&mut self, other: U) {
        *self =  other * self.clone();
    }
}
