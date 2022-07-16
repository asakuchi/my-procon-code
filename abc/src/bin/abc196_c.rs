use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    for i in 1..10usize.pow(6) {
        let text = format!("{}{}", i, i);

        if let Ok(x) = text.parse::<usize>() {
            if x <= n {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
