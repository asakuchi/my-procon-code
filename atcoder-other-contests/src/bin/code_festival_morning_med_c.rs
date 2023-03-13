use ndarray::*;
use proconio::input;

fn main() {
    input! {
        p: f64,
        n: usize,
    }

    // p_0_OFF = 1
    // p_0_ON = 0
    // p_n+1_ON  = p_n_ON * (1-p) + p_n_OFF * p
    // p_n+1_OFF = p_n_ON * p + p_n_OFF * (1-p)

    let b = arr1(&[0., 1.]);
    let c = arr2(&[[1. - p, p], [p, 1. - p]]);

    let result = mat_pow(n, &c, 1).dot(&b);

    println!("{}", result[0]);
}

///
/// 行列累乗
///
fn mat_pow(x: usize, array2: &Array2<f64>, modulus: usize) -> Array2<f64> {
    if x == 0 {
        // 単位行列
        return Array2::eye(2);
    }

    if x == 1 {
        return array2.clone();
    }

    let mut t = mat_pow(x / 2, array2, modulus);

    t = t.dot(&t);
    // t %= modulus;

    if x % 2 == 1 {
        t = t.dot::<Array2<f64>>(&array2);
        // t %= modulus;
    }

    // 正規化
    let p = t[(0, 0)];
    let p_not = t[(0, 1)];

    t *= 1. / (p + p_not);

    t
}
