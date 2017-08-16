extern crate nalgebra as na;

use na::{Matrix1x2, Matrix3x2, Vector4, Vector3, Vector2, Vector1};
use std::f64;

fn main() {}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

#[test]
fn page_49() {
    let x = Vector3::new(-1.0, 1.0, 2.0).map(sigmoid);
    // let x = Vector3::new(-1.0, 1.0, 2.0).sigmoid();
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

// http://qiita.com/eielh/items/5568d2f89924ce3adcef#p26-232-%E9%87%8D%E3%81%BF%E3%81%A8%E3%83%90%E3%82%A4%E3%82%A2%E3%82%B9%E3%81%AE%E5%B0%8E%E5%85%A5
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
    let b = Matrix3x2::new(
        1,2,
        3,4,
        5,6);
    println!("{}", b.nrows()); // => 3
    let (rows, columns) = b.shape();
    println!("{}, {}", rows, columns); // =>s 3, 2
}
