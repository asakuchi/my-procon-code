use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut s: Chars,
    }

    let mut result = 0;

    while let Some(slime) = s.pop() {
        while let Some(next_slime) = s.pop() {
            if slime != next_slime {
                s.push(next_slime);
                break;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
