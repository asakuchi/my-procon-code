use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n],
    }

    s[..k].sort();

    for text in &s[..k] {
        println!("{}", text);
    }
}
