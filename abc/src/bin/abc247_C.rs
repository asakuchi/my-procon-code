use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut memo = vec!["".to_owned(); 20];

    println!("{}", s(n, &mut memo));
}

fn s(n: usize, memo: &mut Vec<String>) -> String {
    if n == 1 {
        return "1".to_owned();
    }

    if memo[n] != "" {
        return memo[n].clone();
    }

    let result = format!("{} {} {}", s(n - 1, memo), n, s(n - 1, memo));

    memo[n] = result.clone();

    result
}
