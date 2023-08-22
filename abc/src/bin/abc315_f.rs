//
// 解説AC
//

use proconio::input;

const MAX_PASS: usize = 100;

fn main() {
    input! {
        n: usize,
        x_y: [(f64, f64); n],
    }

    let mut dp = vec![vec![1_000_000_000_000_f64; MAX_PASS]; n];

    dp[0][0] = 0.;

    for check_point in 0..n - 1 {
        for passed in 0..MAX_PASS {
            for additive_pass in 0..MAX_PASS {
                let next_check_point = check_point + 1 + additive_pass;
                let next_passed = passed + additive_pass;

                if passed + additive_pass >= MAX_PASS {
                    break;
                }

                if next_check_point >= n {
                    break;
                }

                dp[next_check_point][next_passed] = dp[next_check_point][next_passed].min(
                    dp[check_point][passed]
                        + euclidean_distance(x_y[check_point], x_y[next_check_point]),
                )
            }
        }
    }

    let mut penalty = vec![0.; MAX_PASS];

    penalty[1] = 1.;

    for i in 2..MAX_PASS {
        penalty[i] = penalty[i - 1] * 2.;
    }

    let mut result = f64::MAX;

    for passed in 0..MAX_PASS {
        result = result.min(dp[n - 1][passed] + penalty[passed]);
    }

    println!("{}", result);
}

fn euclidean_distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powf(2.) + (a.1 - b.1).powf(2.)).sqrt()
}
