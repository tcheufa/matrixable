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



#[test]
fn test_transform_flipv() {    
    let mut m =  get_square_matrix();
    let mut rot = FlipV::transform(m.clone());
    let mut expected = matrix! [
        [ 6  7  8 ]
        [ 3  4  5 ]
        [ 0  1  2 ]
    ];
    assert_eq!(expected, rot);
    
    assert!(m.into_access(FlipV::access).iter().inspect(|x|println!("{x}")).eq(expected.iter()));
    
    m =  get_vertical_rectangle_matrix();
    rot = FlipV::transform(m);
    expected = matrix![
        [ 9  10  11 ]
        [ 6   7   8 ]
        [ 3   4   5 ]
        [ 0   1   2 ] 
    ];
    assert_eq!(expected, rot);
    
    assert_eq!(FlipV::transform(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transform_fliph() {    
    let mut m =  get_square_matrix();
    let mut rot = FlipH::transform(m);
    let mut expected = matrix! [
        [ 2  1  0 ]
        [ 5  4  3 ]
        [ 8  7  6 ]
    ];
    assert_eq!(expected, rot);
    
    m =  get_horizontal_rectangle_matrix();
    rot = FlipH::transform(m);
    expected = matrix![
        [  4   3   2   1   0 ]
        [  9   8   7   6   5 ]
        [ 14  13  12  11  10 ] 
    ];
    assert_eq!(expected, rot);
    
    assert_eq!(FlipH::transform(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transform_transpose() {    
    let mut m =  get_square_matrix();
    let mut trans = Transposition::transform(m);
    let mut expected = matrix! [
        [ 0  3  6 ]
        [ 1  4  7 ]
        [ 2  5  8 ]
    ];
    assert_eq!(expected, trans);
    
    m = get_horizontal_rectangle_matrix();
    trans = Transposition::transform(m);
    expected = matrix![
        [ 0   5  10 ]
        [ 1   6  11 ]
        [ 2   7  12 ]
        [ 3   8  13 ]
        [ 4   9  14 ]
    ];
    println!("rows = {}, cols = {}", trans.num_rows(), trans.c);
    println!("rows = {}, cols = {}", expected.num_rows(), expected.num_cols());
    assert_eq!(expected, trans);
    
    assert_eq!(Transposition::transform(matrix![[0]]), matrix![[0]]);
} 



#[test]
fn test_transform_rotate_left() {    
    let mut m =  get_square_matrix();
    let mut rot = RotationL::transform(m);
    let mut expected = matrix! [
        [ 2  5  8 ]
        [ 1  4  7 ]
        [ 0  3  6 ]
    ];
    assert_eq!(expected, rot);
    
    m =  get_horizontal_rectangle_matrix();
    rot = FlipH::transform(m);
    expected = matrix![
        [ 3   7  11 ]
        [ 2   6  10 ]
        [ 1   5  9  ]
        [ 0   4  8  ]
    ];
    
    assert!(expected.iter().eq(rot.iter()));
    
    assert_eq!(RotationL::transform(matrix![[0]]), matrix![[0]]);
}


#[test]
fn test_transform_rotate_right() {    
    let mut m =  get_square_matrix();
    let mut rot = RotationR::transform(m);
    let mut expected = matrix! [
        [ 6  3  0 ]
        [ 7  4  1 ]
        [ 8  5  2 ]
    ];
    assert_eq!(expected, rot);
    
//     m =  get_horizontal_rectangle_matrix();
//     rot = FlipH::transform(m);
//     expected = matrix![
//         [  3   2   1   0 ]
//         [  7   6   5   4 ]
//         [ 11  10   9   8 ]
//         [ 15  14  13  12 ] 
//     ];
//     assert_eq!(expected, rot);
//     
    assert_eq!(RotationR::transform(matrix![[0]]), matrix![[0]]);
} 


#[test]
fn test_transform_shiftfront() {    
    let mut m =  get_square_matrix();
    let mut rot = ShiftFront::<2>::transform(m);
    let mut expected = matrix! [
        [ 7  8  0 ]
        [ 1  2  3 ]
        [ 4  5  6 ]
    ];
    assert_eq!(expected, rot);
    
//     m =  get_horizontal_rectangle_matrix();
//     rot = FlipH::transform(m);
//     expected = matrix![
//         [  3   2   1   0 ]
//         [  7   6   5   4 ]
//         [ 11  10   9   8 ]
//         [ 15  14  13  12 ] 
//     ];
//     assert_eq!(expected, rot);
//     
    assert_eq!(ShiftFront::<2>::transform(matrix![[0]]), matrix![[0]]);
} 
