use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut result = Vec::new();

    for i in (0..s.len()).step_by(2) {
        result.push(s[i + 1]);
        result.push(s[i]);
    }

    let text: String = result.iter().collect();

    println!("{}", text);
}
