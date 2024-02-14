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
fn get_vertical_rectangle_matrix() -> MatrixView<u8> {
    matrix![
            [ 0  1  2 ]
            [ 3  4  5 ]
            [ 6  7  8 ]
            [ 9 10 11 ]
    ]
}
fn get_horizontal_rectangle_matrix() -> MatrixView<u8> {
    matrix![
            [  0  1  2  3  4 ]
            [  5  6  7  8  9 ]
            [ 10 11 12 13 14 ]
    ]
}

fn test_transformation (
    get_matrix: fn()->MatrixView<u8>,
    from: fn(MatrixView<u8>) -> MatrixView<u8>,
    expected: MatrixView<u8>)
{
    let t = from(get_matrix());
    assert_eq!(expected, t);
}

#[test]
fn test_transformation_flipv() {   
    test_transformation(get_square_matrix, FlipV::out_of, 
        matrix! [
            [ 6  7  8 ]
            [ 3  4  5 ]
            [ 0  1  2 ]
        ]
    );
    test_transformation(get_vertical_rectangle_matrix, FlipV::out_of,
        matrix![
            [ 9  10  11 ]
            [ 6   7   8 ]
            [ 3   4   5 ]
            [ 0   1   2 ] 
        ]
    );
    assert_eq!(FlipV::out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_fliph() {    
    test_transformation(get_square_matrix, FlipH::out_of,
        matrix! [
            [ 2  1  0 ]
            [ 5  4  3 ]
            [ 8  7  6 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, FlipH::out_of,
        matrix![
            [  4   3   2   1   0 ]
            [  9   8   7   6   5 ]
            [ 14  13  12  11  10 ] 
        ]
    );
    assert_eq!(FlipH::out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_transpose() {   
    test_transformation(get_square_matrix, Transpose::out_of,
        matrix! [
            [ 0  3  6 ]
            [ 1  4  7 ]
            [ 2  5  8 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, Transpose::out_of,
        matrix![
            [ 0   5  10 ]
            [ 1   6  11 ]
            [ 2   7  12 ]
            [ 3   8  13 ]
            [ 4   9  14 ]
        ]
    );
    assert_eq!(Transpose::out_of(matrix![[0]]), matrix![[0]]);
} 



#[test]
fn test_transformation_rotate_left() {    
    test_transformation(get_square_matrix, RotateL::out_of,
        matrix! [
            [ 2  5  8 ]
            [ 1  4  7 ]
            [ 0  3  6 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, RotateL::out_of,
        matrix![
            [ 4   9   14 ]
            [ 3   8   13 ]
            [ 2   7   12 ]
            [ 1   6   11 ]
            [ 0   5   10 ]
        ]
    );
    assert_eq!(RotateL::out_of(matrix![[0]]), matrix![[0]]);
}


#[test]
fn test_transformation_rotate_right() {    
    test_transformation(get_square_matrix, RotateR::out_of,
        matrix! [
            [ 6  3  0 ]
            [ 7  4  1 ]
            [ 8  5  2 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, RotateR::out_of,
        matrix![
            [ 10   5   0 ]
            [ 11   6   1 ]
            [ 12   7   2 ]
            [ 13   8   3 ]
            [ 14   9   4 ]    
        ]
    );
    assert_eq!(RotateR::out_of(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transformation_shift_front() {    
    test_transformation(get_square_matrix, ShiftFront::<2>::out_of,
        matrix! [
            [ 7  8  0 ]
            [ 1  2  3 ]
            [ 4  5  6 ]
        ]
    );
    test_transformation(get_square_matrix, ShiftFront::<3>::out_of,
        matrix! [
            [ 6  7  8 ]
            [ 0  1  2 ]
            [ 3  4  5 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, ShiftFront::<3>::out_of,
        matrix! [
            [ 12 13 14  0  1 ]
            [  2  3  4  5  6 ]
            [  7  8  9 10 11 ]
        ]
    );
    assert_eq!(ShiftFront::<2>::out_of(matrix![[0]]), matrix![[0]]);
    assert_eq!(ShiftFront::<0>::out_of(matrix![[0]]), matrix![[0]]);
} 

#[test]
fn test_transformation_shift_back() {    
    test_transformation(get_square_matrix, ShiftBack::<2>::out_of,
        matrix! [
            [ 2  3  4 ]
            [ 5  6  7 ]
            [ 8  0  1 ]
        ]
    );
    test_transformation(get_square_matrix, ShiftBack::<3>::out_of,
        matrix! [
            [ 3  4  5 ]
            [ 0  1  2 ]
            [ 0  1  2 ]
        ]
    );
    test_transformation(get_horizontal_rectangle_matrix, ShiftBack::<3>::out_of,
        matrix! [
            [  3  4  5  6  7 ]
            [  8  9 10 11 12 ]
            [ 13 14  0  1  2 ]
        ]
    );
    assert_eq!(ShiftBack::<2>::out_of(matrix![[0]]), matrix![[0]]);
    assert_eq!(ShiftBack::<0>::out_of(matrix![[0]]), matrix![[0]]);
} 
