#[cfg(test)]

use crate::traits::{Matrix, MatrixMut, SwapDimensions};
use crate::view::MatrixView;
use crate::matrix;

macro_rules! test_props {
        ($mat:expr, $r:literal, $c:literal, $data:expr) => {{
                let m = $mat;
                assert_eq!($r, m.num_rows());
                assert_eq!($c, m.num_cols());
                assert_eq!($c, m.row_len());
                assert_eq!($r, m.col_len());
                assert_eq!(m.data().len(), m.size());
                assert_eq!($data, m.data());
                m
        }}
}

#[test]
fn test_new () {
        let m = MatrixView::<u8>::with_capacity(1, 2);
        assert_eq!(1, m.num_rows());
        assert_eq!(2, m.num_cols());
        assert_eq!(2, m.row_len());
        assert_eq!(1, m.col_len());
        assert_eq!(2, m.size());
        assert_eq!(0, m.data().len());
        assert_eq!(m.data(), &Vec::<u8>::with_capacity(2));
    
        test_props!( MatrixView::<u8>::with_init(0, 1, 2), 1, 2, &vec![0, 0]);

        test_props!(MatrixView::new(vec![1, 2, 3], 1, 3), 1, 3, &vec![1, 2, 3]);

        // let m1 = MatrixView::<u8>::with_init(5, 2, 2);
        // let m2: MatrixView<u32> = MatrixView::from_try(m1);
        // assert_eq!(m2, Ok(matrix![5u32; 2, 2]));

        // TODO: Add test for from_slice
}

// #[test]
// fn test_from_impl() {
//         let m = MatrixView::try_from(vec![[1; 5]; 4]);
//         assert_eq!(matrix![[1; 5]; 4], m);
//         let m = MatrixView::from([[4; 5]; 4]);
// }

#[test]
fn test_macro() {
        //1
        let m = test_props!(
                matrix![4; 1, 2],
                1, 2,
                &vec![4, 4]
        );
        assert_eq!(MatrixView::with_init(4, 1, 2), m);

        //2
        let m = test_props!(
                matrix!(vec![1, 2, 3, 4, 5, 6], 2, 3),
                2, 3,
                &vec![1, 2, 3, 4, 5, 6]
        );
        assert_eq!(MatrixView::new(vec![1, 2, 3, 4, 5, 6], 2, 3), m);

        //3
        let m = test_props!(
                matrix![ [1, 2], [3, 4], [5, 6], [7, 8],],
                4, 2,
                &vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(matrix!(vec![1, 2, 3, 4, 5, 6, 7, 8], 4, 2), m);

        //4
        let m = test_props!(
                matrix!( (1, 2), (3, 4), (5, 6), (7, 8) ),
                4, 2,
                &vec![1, 2, 3, 4, 5, 6, 7, 8]
        );
        assert_eq!(matrix![vec![1, 2, 3, 4, 5, 6, 7, 8], 4, 2], m);

        //7
        let m = test_props!(
                matrix!([1, 2]; 3),
                3, 2,
                &vec![1, 2, 1, 2, 1, 2]
        );
        assert_eq!(matrix![vec![1, 2, 1, 2, 1, 2], 3, 2], m);


        //8
        let m = test_props!(
                matrix!((1, 2); 3),
                3, 2,
                &vec![1, 2, 1, 2, 1, 2]
        );
        assert_eq!(matrix![vec![1, 2, 1, 2, 1, 2], 3, 2], m);

        //9
        // let v = vec![1, 2];
        // let m = test_props!(
        //         matrix![v; 3],
        //         3, 2,
        //         &vec![1, 2, 1, 2, 1, 2]
        // );
        // assert_eq!(matrix![vec![1, 2, 1, 2, 1, 2], 3, 2], m);


        //10
        // let mut m = matrix!(2, 2);
        // m.fill('a');

        // assert_eq!(2, m.num_rows());
        // assert_eq!(2, m.num_cols());
        // assert_eq!(&vec!['a', 'a', 'a', 'a'], m.data());
        // assert_eq!(matrix![vec!['a'; 4], 2, 2], m);
}

#[test]
fn test_ops() {
        // Operations with primitive types

        let m1 = matrix!(vec![1, 2, 3, 4], 2, 2);
        let m2 = matrix![2; 2, 2];

        // Add
        let mut  p = m1.clone() + m2.clone();
        assert_eq!(p, matrix![[3, 4], [5, 6]]);

        // Sub
        p = m1.clone() - m2.clone();
        assert_eq!(p, matrix![[-1, 0], [1, 2]]);

        // Mul with equal rows and cols
        p = m1 * m2;
        assert_eq!(p, matrix![[6, 6], [14, 14]]);

        // Mul with not equal rows and cols
        p = matrix![[1, 2, 3], [4, 5, 6]] * matrix![[1, 2], [3, 4], [5, 6]];
        assert_eq!(p, matrix![[22, 28], [49, 64]]);

        // PartialEq
        assert!(matrix!(0; 2, 2) == matrix!([0, 0], [0, 0]));
        assert!(matrix!(0; 1, 2) == matrix!(0; 2, 1));


        // Operation with custom types
        #[derive(Debug, PartialEq, Copy, Clone)]
        struct A;

        #[derive(Debug, PartialEq, Copy, Clone)]
        struct B;

        #[derive(Debug, PartialEq, Copy, Clone)]
        struct C;

        impl std::ops::Mul<B> for A {
                type Output = C;
                fn mul(self, _rhs: B) -> Self::Output { C }
        }
        impl std::ops::Add<C> for C {
                type Output = C;
                fn add(self, _rhs: C) -> Self::Output { C }
        }
        impl std::ops::AddAssign<C> for C {
                fn add_assign(&mut self, _rhs: C) {  }
        }
        impl std::ops::Add<B> for A {
                type Output = B;
                fn add(self, _rhs: B) -> Self::Output { B }
        }
        impl std::ops::Add<A> for A {
                type Output = A;
                fn add(self, _rhs: A) -> Self::Output { A }
        }

        let a = matrix![A; 3, 3];
        let b = matrix![B; 3, 3];

        assert_eq!(matrix![B; 3, 3], a.clone() + b.clone());

        assert_eq!(matrix![A; 3, 3], a.clone() + a.clone());

        assert_eq!(matrix![C; 3, 3], a * b);
}


#[test]
fn test_get_set() {
        let mut m = MatrixView::new(vec![1, 2, 3], 1, 3);
        assert_eq!(Some(&3), m.get(0, 2));
        
        let el = m.get_mut(0, 2).unwrap();
        assert_eq!(&mut 3, el);
        *el = 20;
        assert_eq!(Some(&20), m.get(0, 2));
        
        assert_eq!(Ok(()), m.set((0, 2), 100));
        assert_eq!(Some(&100), m.get(0, 2));
        

        assert_eq!(None, m.get(1, 0));
        assert_eq!(None, m.get_mut(1, 0));
        assert_eq!(Err("Cannot access element from indexes."), m.set((1, 0), 11));
        
}


#[test]
fn test_swap_dimensions() {
        let mut m = matrix![0; 2, 2];
        m.swap_dimensions();
        assert_eq!(2, m.num_rows());
        assert_eq!(2, m.num_cols());

        let mut m = matrix![0; 3, 2];
        assert_eq!(None, m.get(1, 2));
        m.swap_dimensions();
        assert_eq!(2, m.num_rows());
        assert_eq!(3, m.num_cols());
        assert_eq!(Some(&0), m.get(1, 2));
}

#[test]
fn test_swaps() {
        let mut m = matrix![
                [(0,0), (0,1), (0,2)],
                [(1,0), (1,1), (1,2)],
                [(2,0), (2,1), (2,2)]
        ];

        m.swap((0,1), (2, 2));
        assert_eq!(m, matrix![
                [(0,0), (2,2), (0,2)],
                [(1,0), (1,1), (1,2)],
                [(2,0), (2,1), (0,1)]
        ]);
        
        m.swapn(2, 7);
        assert_eq!(m, matrix![
                [(0,0), (2,2), (2,1)],
                [(1,0), (1,1), (1,2)],
                [(2,0), (0,2), (0,1)]
        ]);
        
}

#[test]
fn test_one_dimension() {
        assert!(matrix!(0_u8; 1, 1).is_one_dimension());
        assert!(matrix!(0_u8; 2, 1).is_one_dimension());
        assert!(matrix!(0_u8; 1, 2).is_one_dimension());
        assert!(! matrix!(0_u8; 2, 2).is_one_dimension());
}

#[test]
fn test_is_square() {
        // unit
        assert!(matrix![[1]].is_square());

        // row
        assert!(! matrix![[1, 2, 3]].is_square());

        // column
        assert!(! matrix![[0], [1], [3]].is_square());

        // square
        assert!(matrix![0; 4, 4].is_square());

        // any other
        assert!(!matrix![0; 2, 4].is_square());

}

#[test]
fn test_is_symmetric() {
        assert!(matrix!([[0]]).is_symmetric());
        assert!(matrix!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]).is_symmetric());
        assert!(matrix!([[1], [2], [3]]).is_symmetric());
        assert!(matrix!([[1, 2], [2, 3], [3, 4]]).is_symmetric());
}

#[test]
fn test_is_singleton() {
    assert!(matrix![[0]].is_singleton());
    assert!(!matrix![[0][0]].is_singleton());
    assert!(!matrix![[0, 0]].is_singleton());
}

#[test]
fn test_is_horizontal_vertical() {
    assert!(matrix![[0]].is_horizontal());
    assert!(matrix![[0]].is_vertical());
    assert!(matrix![[0, 0]].is_horizontal());
    assert!(!matrix![[0, 0]].is_vertical());
    assert!(!matrix![[0][0]].is_horizontal());
    assert!(matrix![[0][0]].is_vertical());
}

#[test]
fn test_is_diagonal() {
    let m = matrix![
        [1, 0, 0]
        [0, 2, 0]
        [0, 0, 3]
    ];
    
    assert_eq!((true, Some(&0)), m.is_diagonal());
    
    assert_eq!((true, None), matrix![[1]].is_diagonal());
    
    assert_eq!((false, None), matrix![[1][0][2]].is_diagonal());
}
