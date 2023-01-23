use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; 2],
    }

    let mut result = 0;

    for i in 0..n {
        result += a[1][i];
    }

    result += a[0][0];

    let mut score = result;

    for i in 0..n - 1 {
        score -= a[1][i];
        score += a[0][i + 1];

        result = result.max(score);
    }

    println!("{}", result);
}
