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

* Updated `README` file: added properties display.

