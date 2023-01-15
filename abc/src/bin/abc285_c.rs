use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut result = 0usize;

    s.reverse();

    for i in 0..s.len() {
        let c = s[i];

        let x = (c as usize - 'A' as usize + 1) * 26usize.pow(i as u32);

        result += x;
    }

    println!("{}", result);
}
