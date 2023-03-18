use ndarray::*;
// use num_traits::identities::One;
use proconio::input;

use ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_org: [mint; n * n],
    }

    let a = Array::from_shape_vec((n, n), a_org).unwrap();

    let k_array = mat_pow(k, &a);

    let mut result = mint::new(0);

    for i in 0..n {
        for j in 0..n {
            result += k_array[(i, j)];
        }
    }

    println!("{}", result);
}

///
/// 行列累乗
///
fn mat_pow(x: usize, array2: &Array2<mint>) -> Array2<mint> {
    if x == 0 {
        // 単位行列
        return Array2::eye(array2.dim().0);
    }

    if x == 1 {
        return array2.clone();
    }

    let mut t = mat_pow(x / 2, array2);

    t = t.dot(&t);

    if x % 2 == 1 {
        t = t.dot::<Array2<mint>>(&array2);
    }

    t
}
