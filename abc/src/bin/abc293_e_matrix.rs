//!
//! 行列累乗
//!

use ndarray::*;
use proconio::input;

fn main() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    }

    let b = arr1(&[1, 1]);
    let c = arr2(&[[a, 1], [0, 1]]);

    let result = mat_pow(x - 1, &c, m).dot(&b) % m;

    println!("{}", result[0]);
}

fn mat_pow(x: usize, array2: &Array2<usize>, modulus: usize) -> Array2<usize> {
    if x == 0 {
        // 単位行列
        return Array2::eye(2);
    }

    if x == 1 {
        return array2.clone();
    }

    let mut t = mat_pow(x / 2, array2, modulus);

    t = t.dot(&t);
    t %= modulus;

    if x % 2 == 1 {
        t = t.dot::<Array2<usize>>(&array2);
        t %= modulus;
    }

    t
}

