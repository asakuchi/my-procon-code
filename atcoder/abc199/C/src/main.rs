use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::mem;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
    }

    let mut first = s[0..n].to_vec();
    let mut second = s[n..2 * n].to_vec();

    for _ in 0..q {
        input! {
            t: usize,
            a: usize,
            b: usize,
        }

        match t {
            1 => {
                let a = a - 1;
                let b = b - 1;

                if b < n {
                    first.swap(a, b);
                } else if a >= n {
                    second.swap(a - n, b - n);
                } else {
                    mem::swap(&mut first[a], &mut second[b - n]);
                }
            }
            _ => {
                mem::swap(&mut first, &mut second);
            }
        }
    }

    let first_text: String = first.iter().collect();
    let second_text: String = second.iter().collect();

    println!("{}{}", first_text, second_text);
}
