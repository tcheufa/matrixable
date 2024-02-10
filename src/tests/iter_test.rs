use std::ops::{AddAssign, MulAssign};
use crate::prelude::*;
use crate::*;

#[test]
fn test_enumerate() {
        let m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut en = m.enumerate();

        assert_eq!(Some((0, 0, &1)), en.next());
        assert_eq!(Some((0, 1, &2)), en.next());
        assert_eq!(Some((1, 0, &3)), en.next());
        assert_eq!(Some((1, 1, &4)), en.next());
        assert_eq!(Some((2, 0, &5)), en.next());
        assert_eq!(Some((2, 1, &6)), en.next());
        assert_eq!(None, en.next());

        let mut  m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut en = m.enumerate_mut();

        assert_eq!(Some((0, 0, &mut 1)), en.next());
        assert_eq!(Some((0, 1, &mut 2)), en.next());
        assert_eq!(Some((1, 0, &mut 3)), en.next());
        assert_eq!(Some((1, 1, &mut 4)), en.next());
        assert_eq!(Some((2, 0, &mut 5)), en.next());
        assert_eq!(Some((2, 1, &mut 6)), en.next());
        assert_eq!(None, en.next());
}

#[test]
fn test_row() {
        let m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut row = m.row(2).unwrap();
        assert_eq!(Some(&5), row.next());
        assert_eq!(Some(&6), row.next());
        assert_eq!(None, row.next());

        assert!(m.row(3).is_none());

        let mut m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut row = m.row_mut(1).unwrap();
        row.nth(1).unwrap().add_assign(5);

        assert_eq!(None, row.next());
        assert_eq!(matrix![[1, 2], [3, 9], [5, 6]], m);

        assert!(m.row_mut(3).is_none());
}

#[test]
fn test_column() {
        let m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut col = m.column(1).unwrap();
        assert_eq!(Some(&2), col.next());
        assert_eq!(Some(&4), col.next());
        assert_eq!(Some(&6), col.next());
        assert_eq!(None, col.next());

        assert!(m.column(2).is_none());

        let mut m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut col = m.column_mut(1).unwrap();
        col.nth(2).unwrap().mul_assign(5);

        assert_eq!(None, col.next());
        assert_eq!(matrix![[1, 2], [3, 4], [5, 30]], m);

        assert!(m.column_mut(2).is_none());
}

#[test]
/// Using a square matrix.
fn test_diago1() {
        let mut m = matrix![
                [1, 4, 6],
                [7, 2, 5],
                [9, 8, 3]
        ];

        // After the main diagonal.
        {
            let mut diago = m.diag(3).unwrap();
            assert_eq!(Some(&4), diago.next());
            assert_eq!(Some(&5), diago.next());
            assert_eq!(None, diago.next());
        }
        
        // At the main diagonal
        {
            let mut diag_mut = m.diag_mut(2).unwrap();
            assert_eq!(Some(&mut 1), diag_mut.next());
            assert_eq!(Some(&mut 2), diag_mut.next());
            assert_eq!(Some(&mut 3), diag_mut.next());
            assert_eq!(None, diag_mut.next());
        }
        
        // Before the main diagonal.
        {
            let mut diag = m.diag(1).unwrap();
            assert_eq!(Some(&7), diag.next());
            assert_eq!(Some(&8), diag.next());
            assert_eq!(None, diag.next());
        }
        
        // Edges
        {
            let mut first_diag = m.diag(0).unwrap();
            assert_eq!(Some(&9), first_diag.next());
            assert_eq!(None, first_diag.next());
            
            let mut last_diag = m.diag(4).unwrap();
            assert_eq!(Some(&6), last_diag.next());
            assert_eq!(None, last_diag.next());
        }
        
        // Out of bounds
        {
            assert!(m.diag(5).is_none());
        }               

}

#[test]
/// Using a rectangle matrix.
fn test_diago2() {
        let mut m = matrix![
                [1    2   3],
                [4    5   6],
                [7    8   9],
                [10  11  12],
                [13  14  15],
        ];

        // After the consecutive diagonals of same size.
        {
            let mut diago = m.diag(5).unwrap();
            assert_eq!(Some(&2), diago.next());
            assert_eq!(Some(&6), diago.next());
            assert_eq!(None, diago.next());
        }
        
        // At diagonals of same size 3 (3 consecutive diagonals of size 3)
        {
            let mut diag_mut = m.diag_mut(2).unwrap();
            assert_eq!(Some(&mut 7), diag_mut.next());
            assert_eq!(Some(&mut 11), diag_mut.next());
            assert_eq!(Some(&mut 15), diag_mut.next());
            assert_eq!(None, diag_mut.next());
            
            let mut diag_mut = m.diag_mut(3).unwrap();
            assert_eq!(Some(&mut 4), diag_mut.next());
            assert_eq!(Some(&mut 8), diag_mut.next());
            assert_eq!(Some(&mut 12), diag_mut.next());
            assert_eq!(None, diag_mut.next());
            
            let mut diag_mut = m.diag_mut(4).unwrap();
            assert_eq!(Some(&mut 1), diag_mut.next());
            assert_eq!(Some(&mut 5), diag_mut.next());
            assert_eq!(Some(&mut 9), diag_mut.next());
            assert_eq!(None, diag_mut.next());
        }
        
        // Before the main diagonal.
        {
            let mut diag = m.diag(1).unwrap();
            assert_eq!(Some(&10), diag.next());
            assert_eq!(Some(&14), diag.next());
            assert_eq!(None, diag.next());
        }
        
        // Edges
        {
            let mut first_diag = m.diag(0).unwrap();
            assert_eq!(Some(&13), first_diag.next());
            assert_eq!(None, first_diag.next());
            
            let mut last_diag = m.diag(6).unwrap();
            assert_eq!(Some(&3), last_diag.next());
            assert_eq!(None, last_diag.next());
        }
        
        // Out of bounds
        {
            assert!(m.diag(7).is_none());
        }               

}

#[test]
fn test_rows() {
        let mut m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut rows = m.rows();
        assert_eq!(vec![&1, &2], rows.next().unwrap().collect::<Vec<_>>());
        assert_eq!(vec![&3, &4], rows.next().unwrap().collect::<Vec<_>>());
        assert_eq!(vec![&5, &6], rows.next().unwrap().collect::<Vec<_>>());
        assert!(rows.next().is_none());

        let mut rows = m.rows_mut();
        let row = rows.nth(2).unwrap();
        row.for_each(|x| *x *= -10);
        assert!(rows.next().is_none());

        assert_eq!(matrix![[1, 2], [3, 4], [-50, -60]], m);
}

#[test]
fn test_columns() {
        let mut m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut cols = m.columns();
        assert_eq!(vec![&1, &3, &5], cols.next().unwrap().collect::<Vec<_>>());
        assert_eq!(vec![&2, &4, &6], cols.next().unwrap().collect::<Vec<_>>());
        assert!(cols.next().is_none());

        let mut cols = m.columns_mut();
        let col = cols.nth(1).unwrap();
        col.for_each(|x| *x *= -10);
        assert!(cols.next().is_none());

        assert_eq!(matrix![[1, -20], [3, -40], [5, -60]], m);
}

// INCOMPLETE ! MUST HAVE TESTS FOR ALL ITERATORS
//         #[test]
//         fn test_from_iterator() {
//                 let mut m = matrix![
//                         [1, 2],
//                         [3, 4],
//                         [5, 6]
//                 ];
// 
//                 // If `FromIterator` was not implemented
//                 let rows: Vec<Vec<_>> = m.rows().map(|r|r.collect::<Vec<_>>()).collect();
//                 assert_eq!(vec![
//                         vec![&1, &2],
//                         vec![&3, &4],
//                         vec![&5, &6]], rows);
// 
//                 //  With `FromIterator` trait
//                 let cols: Vec<Vec<_>> = m.columns().collect();
//                 //let cols: Vec<_> = cols.into_iter().flatten().collect();
//                 assert_eq!(vec![
//                         vec![&1, &3, &5],
//                         vec![&2, &4, &6]], cols);
// 
//                 // Explicity using the `from_iter()` trait method
//                 {
//                         let rows_mut = Vec::<Vec<_>>::from_iter(m.rows_mut());
//                         assert_eq!(vec![
//                                 vec![&mut 1, &mut 2],
//                                 vec![&mut 3, &mut 4],
//                                 vec![&mut 5, &mut 6]], rows_mut);
//                 }
//                 let cols_mut = Vec::<Vec<_>>::from_iter(m.columns_mut());
//                 assert_eq!(vec![
//                         vec![&mut 1, &mut 3, &mut 5],
//                         vec![&mut 2, &mut 4, &mut 6]], cols_mut);
// 
// 
//                 let into_rows: Vec<Vec<_>> = m.clone().into_rows().collect();
//                 assert_eq!(vec![
//                         vec![1, 2],
//                         vec![3, 4],
//                         vec![5, 6]], into_rows);
// 
//                 let into_cols: Vec<Vec<_>> = m.clone().into_cols().collect();
//                 assert_eq!(vec![
//                         vec![1, 3, 5],
//                         vec![2, 4, 6]], into_cols);
// 
//                 let into_diagos: Vec<Vec<_>> = m.clone().into_diags().collect();
//                 assert_eq!(vec![
//                            vec![5],
//                         vec![3, 6],
//                         vec![1, 4],
//                           vec![2]], into_diagos);
// 
//         }

//         #[test]
//         fn test_extend_impl() {
//                 let mut m = matrix![
//                                 [ 1   2 ]
//                                 [ 3   4 ]
//                                 [ 5   6 ]
//                 ];
// 
//                 let mut m2 = m.clone();
// 
//                 let adder = matrix![
//                                 [  7   8   9 ]
//                                 [ 10  11  12 ]
//                                 [ 13  14  15 ]
//                 ];
// 
//                 let rows_adder = adder.clone().into_rows();
//                 let cols_adder = adder.into_cols();
// 
//                 let expected_for_rows_add = matrix![
//                                 [  1    2  ]
//                                 [  3    4  ] //   v   v   x
//                                 [  5    6  ] //  [0] [1] [2] out of bounds.
//                                 [  7   10  ] // [ 7, 10, |13] => [7, 10] --- 13 is ignored and then dropped.
//                                 [  8   11  ] // [ 8, 11, |14] => [8, 11] --- 14 ignored and dropped.
//                                 [  9   12  ] // [ 9, 12, |15] => [9, 12] --- 15 ignored and dropped.
//                 ];
// 
//                 let expected_for_cols_add = matrix![
//                                 [ 1  2  7  10  13 ]
//                                 [ 3  4  8  11  14 ]
//                                 [ 5  6  9  12  15 ]
//                 ];
// 
//                 // Before
//                 // let extender = cols_adder.map(|a| Extender::new(a, Extend::Row));
//                 // m.extend(extender);
//                 // assert_eq!(expected_for_rows_add, m);
// 
//                 // let extender2 = rows_adder.map(|a| Extender::new(a, Extend::Column));
//                 // m2.extend(extender2);
//                 // assert_eq!(expected_for_cols_add, m2);
// 
// 
//                 // Now
//                 //
//                 m.extend(MultiExtender::Row(cols_adder));
//                 assert_eq!(expected_for_rows_add, m);
// 
//                 m2.extend(MultiExtender::Column(rows_adder));
//                 assert_eq!(expected_for_cols_add, m2);
// 
//         }

#[test]
fn test_iter_applications() {
        let mut m = matrix![[1, 2], [3, 4], [5, 6]];
        let mut columns = m.columns_mut();

        let col0 = columns.next().unwrap();
        let col1 = columns.next().unwrap();

        col0.for_each(|x| *x += 1);
        col1.for_each(|x| *x -= 2);

        assert_eq!(matrix![[2, 0], [4, 2], [6, 4]], m);

}

