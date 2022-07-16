use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;

    for i in 0..s.len() {
        let mut length = 0;

        for j in i..s.len() {
            if s[j] == 'A' || s[j] == 'C' || s[j] == 'G' || s[j] == 'T' {
                length += 1;
            } else {
                break;
            }
        }

        result = std::cmp::max(result, length);
    }

    println!("{}", result);
}
