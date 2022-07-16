use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        mut x: usize,
        s: Chars,
    }

    let mut stack = Vec::new();

    for c in s {
        if c == 'U' {
            if let Some(top) = stack.pop() {
                if top == 'L' || top == 'R' {
                    // drop data
                    continue;
                } else {
                    stack.push(top);
                }
            }
        }
        stack.push(c);
    }

    for c in stack.iter() {
        match c {
            'U' => {
                x /= 2;
            }
            'L' => {
                x *= 2;
            }
            _ => {
                x = 2 * x + 1;
            }
        }
    }

    println!("{}", x);
}
