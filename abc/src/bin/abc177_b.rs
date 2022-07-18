use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut result = 100000;

    for i in 0..s.len() - t.len() + 1 {
        let mut diff = 0;

        for j in 0..t.len() {
            if s[i + j] != t[j] {
                diff += 1;
            }
        }

        result = result.min(diff);
    }

    println!("{}", result);
}
