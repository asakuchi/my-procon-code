use ::procon_library_rs::run_length_encoding::run_length_encoding;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let list = run_length_encoding(&s);

    println!("{}", list.len());
}
