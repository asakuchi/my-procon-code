use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut result = 0;

    for i in 1..=n {
        let sum = i
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .sum();

        if a <= sum && sum <= b {
            result += i;
        }
    }

    println!("{}", result);
}
