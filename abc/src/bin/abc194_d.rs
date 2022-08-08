use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // オープンコレクター問題
    let mut result = 0.;

    for i in 2..=n {
        result += n as f64 / (n - i + 1) as f64;
    }

    println!("{}", result);
}
