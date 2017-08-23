extern crate nalgebra as na;

use na::{Matrix1x2, Matrix1x3, Matrix2, Matrix2x3, Matrix3x2, Vector4, Vector3, Vector2, Vector1, RowVector3, RowVector2};
use std::f64;

fn main() {}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

fn identity_function<T>(x: T) -> T {
    x
}

fn forward(
    w1: Matrix2x3<f64>,
    w2: Matrix3x2<f64>,
    w3: Matrix2<f64>,
    b1: Matrix1x3<f64>,
    b2: Matrix1x2<f64>,
    b3: Matrix1x2<f64>,
    x: Matrix1x2<f64>,
) -> Matrix1x2<f64> {
    let a1 = x * w1 + b1;
    let z1 = a1.map(sigmoid);
    let a2 = z1 * w2 + b2;
    let z2 = a2.map(sigmoid);
    let a3 = z2 * w3 + b3;
    identity_function(a3)
}

// 解説
// https://goo.gl/FPm4tq
#[test]
fn page_48() {
    let x = Vector3::new(-1.0, 1.0, 2.0).map(sigmoid);
    println!("{}", x);

    let range = 0.00000001;

    let expect_x1 = 0.26894142;
    assert!(x[0] < expect_x1 + range);
    assert!(x[0] > expect_x1 - range);

    let expect_x2 = 0.73105858;
    assert!(x[1] < expect_x2 + range);
    assert!(x[1] > expect_x2 - range);

    let expect_x3 = 0.88079708;
    assert!(x[2] < expect_x3 + range);
    assert!(x[2] > expect_x3 - range);
}


// 解説
// https://goo.gl/AFMx6b
#[test]
fn page_26() {
    let x = Matrix1x2::new(0.0, 1.0);
    println!("{}", x);
    let w = Vector2::new(0.5, 0.5);
    println!("{}", w);
    let b = Vector1::new(-0.7);
    println!("{}", b);
    let sum = x * w + b;
    println!("{}", sum);

    let expect = Vector1::new(-0.2);
    let range = Vector1::new(0.0001);
    assert!(sum < expect + range);
    assert!(sum > expect - range);
}

// 解説
// https://goo.gl/UKqopj
#[test]
fn page_53_1() {
    let a = Vector4::new(1.0, 2.0, 3.0, 4.0);
    println!("{}", a);

    println!("{}", a.nrows()); // => 4

    let (rows, columns) = a.shape();
    println!("{}, {}", rows, columns); // => 4, 1
}

#[test]
fn page_53_2() {
    let b = Matrix3x2::new(1, 2, 3, 4, 5, 6);
    println!("{}", b.nrows()); // => 3
    let (rows, columns) = b.shape();
    println!("{}, {}", rows, columns); // =>s 3, 2
}

#[test]
fn page_64() {
    let w1 = Matrix2x3::from_rows(
        &[
            RowVector3::new(0.1, 0.3, 0.5),
            RowVector3::new(0.2, 0.4, 0.6),
        ],
    );
    let b1 = Matrix1x3::new(0.1, 0.2, 0.3);
    let w2 = Matrix3x2::from_rows(
        &[
            RowVector2::new(0.1, 0.4),
            RowVector2::new(0.2, 0.5),
            RowVector2::new(0.3, 0.6),
        ],
    );
    let b2 = Matrix1x2::new(0.1, 0.2);
    let w3 = Matrix2::from_rows(&[RowVector2::new(0.1, 0.3), RowVector2::new(0.2, 0.4)]);
    let b3 = Matrix1x2::new(0.1, 0.2);

    let x = Matrix1x2::new(1.0, 0.5);

    let y = forward(w1, w2, w3, b1, b2, b3, x);
    println!(" = {}", y);
    let expect_y1 = 0.31682708;
    let expect_y2 = 0.69627908;
    let range = 0.00000001;
    assert!(y[0] < expect_y1 + range);
    assert!(y[0] > expect_y1 - range);
    assert!(y[1] < expect_y2 + range);
    assert!(y[1] > expect_y2 - range);
}
