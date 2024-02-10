//! Traits other than the `Matrix` trait are all packed here.

/// A trait for collections that can swap their elements places.
pub trait Swap<Idx = usize> {
    /// The index type.
    fn swap(&mut self, a: Idx, b: Idx);
}

/// A trait for collections
pub trait Collection<Elem> {  }

/// Trait for collections that can be iterated over.
pub trait Iterable {
    type Iter<'a>: Iterator where Self: 'a;
    fn iterate(&self) -> Self::Iter<'_>;
}

/// Trait for collections that can be iterated over with mutable access enabled.
pub trait IterableMut: Iterable {
    type IterMut<'a>: Iterator where Self: 'a;
    fn iterate_mut(&mut self) -> Self::IterMut<'_>;
}


/// A Strategy trait which defines access and transformation over matrices.
pub trait TransformStrategy<M: crate::traits::Matrix> {
    //fn clone(m: &M) -> M where &M::Element: Clone;
    fn access(m: &M, i: usize, j: usize) -> Option<(usize, usize)>;
    fn transform(m: M) -> M;
}
