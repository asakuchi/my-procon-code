use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;

    let mut stack = Vec::new();

    for c in s {
        if c == 'S' {
            stack.push(c);
        } else {
            if let Some(_) = stack.pop() {
            } else {
                result += 1;
            }
        }
    }

    result += stack.len();

    println!("{}", result);
}
