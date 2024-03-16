# Version 0.1.1 (March 04, 2024)

* Updated crate's metadata inside `Cargo.toml` file.

# Version 0.1.2 (March 04, 2024)

* Deprecated trait function `MatrixExt::is_one_dimension` in favor of a more concise name that is `MatrixExt::is_vector`.
* Updated `README` file: added library description and the `IdentityMatrix` example.

# Version 0.1.3 (March 05, 2024)

* Made a more prettier output produced by functions `print_rows_debug`, `print_columns_debug` and `print_diagonals_debug`.

* Corrected variable name in `print_diagonals_debug`: ~`cols`~ => `diags`

* Updated example in `README.md` and added output.

* Updated `Cargo.toml` file


# Version 0.2.0 (March 06, 2024)

* Added structs: `IntoRows`, `IntoCols` and `IntoDiags`, and their associated functions: `into_rows`, `into_cols`, `into_diags`.

* Added method `into_iter`.

* Added unchecked (unsafe) version of `row`, `col`, `diag`, `row_mut`, `col_mut` and `diag_mut` methods.

* Put description on `Row`, `Column`, `Diag`, `RowMut`, `ColumnMut` and `DiagMut` (The 1 dimensional iterators).

* Changed manual implementation of `Copy` and `Clone` to derive macro on 1D iterators (hope that will never cause a crash).

* Updated `README` file: added properties display on example.


# Version 0.3.0 (March 14, 2024) 

## 🚨 **BREAKING CHANGES**

* Removed method `into_iter` and changed clause `Into<Vec<Self::Element>>` into `Sized + IntoIterator<Self::Element>` on methods `into_rows`, `into_cols` and `into_diags`.

* Added clause `<I as IntoIterator>::IntoIter: ExactSizeIterator` inside `from_iter` method from `MatrixExtFromIter` trait.

* Removed the `Copy` constraint on generic parameter `T` inside `TransformStrategy<[[T; N]; M]> for Transpose` implementation

* Added `no_std` support, making now use of `std` module optional.


# Version 0.4.0 (March 15, 2024) 

## 🚨 **BREAKING CHANGES**

* Removed method `duplicate` in `MatrixExtMut`'s provided methods.

* Added method `diag_len`.

* Added methods `first` and `first_mut`.

* Added methods `last` and `last_mut`.

* Fixed `num_diags` overflow error: It now returns 0 for an empty matrix.

* Added a `prelude` module.

* Fields of struct `Observer` are now public.

* `InPlace` trait now requires Matrix as `MatrixExtMut` instead of `MatrixExt`.

* Implementation of `DoubleEndedIterator` for iterators `Row(Mut)`, `Column(Mut)`, `Diag(Mut)` now follows the behavior from the rust standard library documentation.

* Added implemention of `TransformStrategy` and `InPlace` for immutable references implementing these traits.

* Changed `Sized + IntoIterator<Self::Element>` clause to `Self: Sized` and `IntoRows<Self::Element>: From<Self>`, `IntoCols<Self::Element>: From<Self>`, `IntoDiags<Self::Element>: From<Self>`, respectively on methods `into_rows`, `into_cols` and `into_diags`.
