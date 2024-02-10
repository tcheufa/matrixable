use super::traits::Matrix;
 
trait ZeroMatrix: Matrix 
where for<'a> 
<<<Self as Matrix>
    ::Data as req::Iterable>
        ::Iter<'a> as Iterator>
            ::Item: PartialEq<<Self as Matrix>::Element>,
Self::Data: Iterable,
{
    const ZERO: Self::Element;
    
    /// Checks if the matrix is square and if all elements of matrix are `zero` except diagonal elements. 
    fn is_diagonal(&self) -> bool {
        if !self.is_square()  { 
            return false
        }
        
        for (i, j, el) in self.enumerate() {
            if i == j {
                if el == Self::ZERO {
                    return false 
                }
            }
            else if el != Self::ZERO { 
                return false 
            }
        }
        true
    }
    
    /// Check if a diagonal matrix have the same elements on its diagonal.
    fn is_scalar(&self) -> bool 
    where for<'a>
    <<<Self as Matrix>
        ::Data as req::Iterable>
            ::Iter<'a> as Iterator>
                ::Item: PartialEq + PartialEq<<Self as Matrix>::Element>,
    {
        if !self.is_square()  { 
            return false
        }
        
        
        let mut it = self.enumerate();
        let (_, _, mut prev) = it.next().expect("At least one element.");
        
        for (i, j, el) in it {
            if i == j {
                if el == Self::ZERO || el != prev {
                    return false 
                }
                prev= el;
            }
            else if el != Self::ZERO { 
                return false 
            }
        }
        true
    }
}


trait IdentityMatrix: ZeroMatrix 
where for<'a>
<<<Self as Matrix>
    ::Data as req::Iterable>
        ::Iter<'a> as Iterator>
            ::Item: PartialEq<<Self as Matrix>::Element>,
Self::Data: Iterable,
{
    const ONE: Self::Element;
        
    fn is_identity(&self) -> bool {
        if !self.is_square()  { 
            return false
        }
        
        for (i, j, el) in self.enumerate() {
            if i == j {
                if el == Self::ZERO || el != Self::ONE {
                    return false 
                }
            }
            else if el != Self::ONE { 
                return false 
            }
        }
        true
    }
}
