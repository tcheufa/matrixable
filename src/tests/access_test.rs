// use std::ops::{AddAssign, MulAssign};
use crate::prelude::*;
use crate::*;

fn get_square_matrix() -> MatrixView<u8> {
    matrix![
            [ 0  1  2 ]
            [ 3  4  5 ]
            [ 6  7  8 ]
    ]
}
fn get_vertical_matrix() -> MatrixView<u8> {
    matrix![
            [ 0  1  2 ]
            [ 3  4  5 ]
            [ 6  7  8 ]
            [ 9 10 11 ]
    ]
}
fn get_horizontal_matrix() -> MatrixView<u8> {
    matrix![
            [  0  1  2  3  4 ]
            [  5  6  7  8  9 ]
            [ 10 11 12 13 14 ]
    ]
}

fn test_transformation<T: Clone + TransformStrategy<MatrixView<u8>>> (
    get_matrix: fn()->MatrixView<u8>,
    transformation: T,
    expected: MatrixView<u8>)
{
    let m = get_matrix();
    
    let access = m.access(transformation.clone());
    print_rows_debug(&expected);
    print_rows_debug(&access);
    
    assert!(expected.iter().eq(access.iter()));
    
    let t = transformation.out_of(m);
    assert_eq!(expected, t);
}



#[test]
fn test_transformation_identity() {
    test_transformation(get_square_matrix, Identity,
        matrix![
                [ 0  1  2 ]
                [ 3  4  5 ]
                [ 6  7  8 ]
        ]
    );

    test_transformation(get_vertical_matrix, Identity,
        matrix![
                [ 0  1  2 ]
                [ 3  4  5 ]
                [ 6  7  8 ]
                [ 9 10 11 ]
        ]
    );
    
    test_transformation(get_horizontal_matrix, Identity,
        matrix![
                [  0  1  2  3  4 ]
                [  5  6  7  8  9 ]
                [ 10 11 12 13 14 ]
        ]
    );
    
    assert_eq!(Identity.out_of(matrix![[0]]), matrix![[0]]);
} 

#[test]
fn test_transformation_transpose() {   
    test_transformation(get_square_matrix, Transpose,
        matrix! [
            [ 0  3  6 ]
            [ 1  4  7 ]
            [ 2  5  8 ]
        ]
    );
    test_transformation(get_horizontal_matrix, Transpose,
        matrix![
            [ 0   5  10 ]
            [ 1   6  11 ]
            [ 2   7  12 ]
            [ 3   8  13 ]
            [ 4   9  14 ]
        ]
    );
    assert_eq!(Transpose.out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_flipv() {   
    test_transformation(get_square_matrix, FlipV, 
        matrix! [
            [ 6  7  8 ]
            [ 3  4  5 ]
            [ 0  1  2 ]
        ]
    );
    test_transformation(get_vertical_matrix, FlipV,
        matrix![
            [ 9  10  11 ]
            [ 6   7   8 ]
            [ 3   4   5 ]
            [ 0   1   2 ] 
        ]
    );
    assert_eq!(FlipV.out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_fliph() {    
    test_transformation(get_square_matrix, FlipH,
        matrix! [
            [ 2  1  0 ]
            [ 5  4  3 ]
            [ 8  7  6 ]
        ]
    );
    test_transformation(get_horizontal_matrix, FlipH,
        matrix![
            [  4   3   2   1   0 ]
            [  9   8   7   6   5 ]
            [ 14  13  12  11  10 ] 
        ]
    );
    assert_eq!(FlipH.out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_reverse() {
    test_transformation(get_square_matrix, Reverse,
        matrix![
            [ 8  7  6 ]
            [ 5  4  3 ]
            [ 2  1  0 ]
        ]
    );
    test_transformation(get_horizontal_matrix, Reverse,
        matrix![
            [ 14 13 12 11 10 ]
            [  9  8  7  6  5 ]
            [  4  3  2  1  0 ]
        ]
    );
    
    assert_eq!(Reverse.out_of(matrix![[0]]), matrix![[0]]);
}


#[test]
fn test_transformation_rotate_left() {    
    test_transformation(get_square_matrix, RotateL,
        matrix! [
            [ 2  5  8 ]
            [ 1  4  7 ]
            [ 0  3  6 ]
        ]
    );
    test_transformation(get_horizontal_matrix, RotateL,
        matrix![
            [ 4   9   14 ]
            [ 3   8   13 ]
            [ 2   7   12 ]
            [ 1   6   11 ]
            [ 0   5   10 ]
        ]
    );
    assert_eq!(RotateL.out_of(matrix![[0]]), matrix![[0]]);
}


#[test]
fn test_transformation_rotate_right() {    
    test_transformation(get_square_matrix, RotateR,
        matrix! [
            [ 6  3  0 ]
            [ 7  4  1 ]
            [ 8  5  2 ]
        ]
    );
    test_transformation(get_horizontal_matrix, RotateR,
        matrix![
            [ 10   5   0 ]
            [ 11   6   1 ]
            [ 12   7   2 ]
            [ 13   8   3 ]
            [ 14   9   4 ]    
        ]
    );
    assert_eq!(RotateR.out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_shift_front() {    
    test_transformation(get_square_matrix, ShiftFront(2),
        matrix! [
            [ 7  8  0 ]
            [ 1  2  3 ]
            [ 4  5  6 ]
        ]
    );
    test_transformation(get_square_matrix, ShiftFront(3),
        matrix! [
            [ 6  7  8 ]
            [ 0  1  2 ]
            [ 3  4  5 ]
        ]
    );
    test_transformation(get_horizontal_matrix, ShiftFront(3),
        matrix! [
            [ 12 13 14  0  1 ]
            [  2  3  4  5  6 ]
            [  7  8  9 10 11 ]
        ]
    );
    assert_eq!(ShiftFront(2).out_of(matrix![[0]]), matrix![[0]]);
    assert_eq!(ShiftFront(0).out_of(matrix![[0]]), matrix![[0]]);
} 

#[test]
fn test_transformation_shift_back() {    
    test_transformation(get_square_matrix, ShiftBack(2),
        matrix! [
            [ 2  3  4 ]
            [ 5  6  7 ]
            [ 8  0  1 ]
        ]
    );
    test_transformation(get_square_matrix, ShiftBack(3),
        matrix! [
            [ 3  4  5 ]
            [ 6  7  8 ]
            [ 0  1  2 ]
        ]
    );
    test_transformation(get_horizontal_matrix, ShiftBack(3),
        matrix! [
            [  3  4  5  6  7 ]
            [  8  9 10 11 12 ]
            [ 13 14  0  1  2 ]
        ]
    );
    assert_eq!(ShiftBack(2).out_of(matrix![[0]]), matrix![[0]]);
    assert_eq!(ShiftBack(0).out_of(matrix![[0]]), matrix![[0]]);
} 

#[test]
fn test_access_submatrix() {
    let m = get_square_matrix();
    let access = m.access(SubMatrix::of(&m, 1, 2));
    let expected = matrix![
        [ 0  1 ]
        [ 6  7 ]
    ];
    assert!(expected.iter().eq(access.iter()));
    
    // This could be useful for test empty matrix property.
    //let m = matrix![[0]];
    //assert!(m.iter().eq(m.access(SubMatrix::of(&m, 0, 0)).iter()));
}


#[test]
fn test_access_map() {
    let m = matrix![
        ['a', 'b', 'c']
        ['d', 'e', 'f']
        ['g', 'h', 'i']
    ];
    
    let mapping = matrix![
        [ 1  0  3  0 ]
        [ 4  5  6  6 ]
        [ 2  8  3  8 ]
    ];
    
    let expected = matrix![
        ['b', 'a', 'd', 'a']
        ['e', 'f', 'g', 'g']
        ['c', 'i', 'd', 'i']
    ];
    
    let access = m.access(AccessMap(mapping));
    
    assert!(expected.iter().eq(access.iter()));
}
