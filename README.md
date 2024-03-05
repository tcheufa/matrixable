# matrixable
A crate providing utilities for matrix manipulation.

This library possesses two main traits: `MatrixExt` and `MatrixMutExt`.

## Example 
```rust
use matrixable::MatrixExt;

struct IdentityMatrix { size: usize }

impl MatrixExt for IdentityMatrix {
    type Element = u32;
    
    fn num_rows(&self) -> usize { self.size }

    fn num_cols(&self) -> usize { self.size }

    fn get(&self, i: usize, j: usize) -> Option<&Self::Element> {
        if i >= self.size || j >= self.size {
            None
        }
        else if i == j {
            Some(&1)
        }
        else {
            Some(&0)
        } 
    }
}

fn main() {
    let identity = IdentityMatrix { size: 3 };
    
    matrixable::print_rows_debug(&identity);
}
```

## MatrixExt 
This trait requires three methods to be implemented:
    * `num_rows`: which should give the number of rows of the matrix.
    * `num_cols`: which should give the the number of columns.
    * `get`: which may return a reference to an element of the matrix.

Once those methods are implemented, the following features automatically become available:
* **immutability**: All the functions provided by this trait does not modify elements of the matrix-like struct unless that struct is consumed in the process.
* **iterators**: iteratate over all elements and also over rows, columns and diagonals.
* **access**: elements differently without changing their positions: transpose access, rotate access, submatrix... 
* **transformation**: transform struct into another type, maybe another matrix.
* **metadata**: obtain information about the matrix: symmetry, dimensions, diagonality...
* and much more !

## MatrixMutExt
This traits requires for the struct to first implement `MatrixExt` as well as its single required method: `get_mut`.

Once implemented `MatrixMutExt` structs inherits features from `MatrixExt` plus the following:
* **mutability**: Functions provided by this trait allow elements to be mutated. Mutable versions of the above features also become available (iterators, access).
* **in-place modification**.


plus an additional one to enable mutability of elements.
See [**this**](https://docs.rs/matrixable/0.1.0/matrixable/) documentation for more information.


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
