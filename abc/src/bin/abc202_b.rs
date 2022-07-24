use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let result = s
        .iter()
        .map(|&c| match c {
            '6' => '9',
            '9' => '6',
            other => other,
        })
        .rev()
        .collect::<String>();

    println!("{}", result);
}
