//!
//! 二次元累積和
//!

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        v: usize,
        a: [[usize; w]; h],
    }

    let mut s = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            s[i + 1][j + 1] += a[i][j] + s[i + 1][j] + s[i][j + 1] - s[i][j];
        }
    }

    let mut result = 0;

    for p_i in 1..=h {
        for p_j in 1..=w {
            for q_i in p_i..=h {
                for q_j in p_j..=w {
                    let area_price =
                        s[q_i][q_j] + s[p_i - 1][p_j - 1] - s[q_i][p_j - 1] - s[p_i - 1][q_j];
                    let area = (q_i - p_i + 1) * (q_j - p_j + 1);
                    let house_price = area * k;

                    let price = area_price + house_price;

                    if price <= v {
                        result = result.max(area);
                    }
                }
            }
        }
    }

    println!("{}", result);
}
