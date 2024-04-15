# matrixable
This crate provides utilities for matrix manipulation.

## Example 
```rust
use matrixable::MatrixExt;

struct IdentityMatrix { dim_size: usize }

impl MatrixExt for IdentityMatrix {
    type Element = i32;

    #[inline]
    fn num_rows(&self) -> usize { self.dim_size }

    #[inline]
    fn num_cols(&self) -> usize { self.dim_size }

    #[inline]
    fn get(&self, i: usize, j: usize) -> Option<&Self::Element> {
        if i >= self.dim_size || j >= self.dim_size {
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
    let identity = IdentityMatrix { dim_size: 3 };

    matrixable::print_rows_debug(&identity);
    println!();

    matrixable::print_columns_debug(&identity);
    println!();

    matrixable::print_diagonals_debug(&identity);
    println!();

    println!("Properties:");
    println!("* Dimensions: {:?}", identity.dimensions());
    println!("* Square matrix: {}", identity.is_square());
    println!("* Symmetric: {}", identity.is_symmetric());
    println!("* Skew-symmetric: {}", identity.is_skew_symmetric());
    println!("* Diagonal matrix: {}", identity.is_diagonal().0);
    println!("* Scalar matrix: {}", identity.is_scalar().0);
    println!("* Constant matrix: {}", identity.is_constant().0);
}
```
Output:
```
Rows
0: [1, 0, 0]
1: [0, 1, 0]
2: [0, 0, 1]

Columns
0: [1, 0, 0]
1: [0, 1, 0]
2: [0, 0, 1]

Diagonals
0: [0]
1: [0, 0]
2: [1, 1, 1]
3: [0, 0]
4: [0]

Properties:
* Dimensions: (3, 3)
* Square matrix: true
* Symmetric: true
* Skew-symmetric: false
* Diagonal matrix: true
* Scalar matrix: true
* Constant matrix: false
```

This library has two main traits: `MatrixExt` and `MatrixMutExt`.

## MatrixExt 

This trait requires three methods to be implemented:

* `num_rows`: which should give the number of rows of the matrix.
* `num_cols`: which should give the the number of columns.
* `get`: which returns a reference to an element of the matrix if found.

Once these methods are implemented, the following features are provided:

* **immutability**: All the functions provided by this trait does not modify elements of the matrix-like struct unless that struct is consumed in the process.
* **iterators**: Iterate over all elements and also over rows, columns and diagonals.
* **access**: Access elements differently without changing their positions: transpose access, rotate access, reshape, sub-matrix... 
* **transformation**: Transform struct into another type, maybe another matrix.
* **metadata**: Get information about the matrix: symmetry, dimensions, diagonality...


## MatrixMutExt

This trait requires for the struct to first implement `MatrixExt`, and then the trait required method: `get_mut`.

Once implemented `MatrixMutExt` structs inherits features from `MatrixExt` plus the following:

* **mutability**: Functions provided by this trait allow elements to be mutated. Mutable versions of the above features also become available (iterators, access).
* **swap**: Swap elements, rows or columns. 
* **in-place modification**.

## Important
* Note also that this crate extends the standard 2D array `[[T; N]; M]`, now available through the `impls` default feature.
  The default features can be disabled by setting `default-features` parameter to `false` as follows:
    ```toml
    matrixable = { version = "0.5.0", default-features = false }
    ```
* This crate support `no_std` environments since ver`0.3.0`.
* Many improvements and changes (breaking changes as well) have been done since the beginning of this project. Please see the [`CHANGELOG`](CHANGELOG.md) for more details.
* Your help and suggestions on improving this crate are highly encouraged. So please feel free to report any issue you may encounter!


## More
More information about the library in the [**documentation**](https://docs.rs/matrixable/0.1.0/matrixable/).

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
