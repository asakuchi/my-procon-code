// use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

// #[fastout]
fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars,
    }

    let s = vec![s1, s2, s3];

    for i in t.iter().map(|c| c.to_string().parse::<usize>().unwrap()) {
        print!("{}", s[i - 1]);
    }

    println!();
}
