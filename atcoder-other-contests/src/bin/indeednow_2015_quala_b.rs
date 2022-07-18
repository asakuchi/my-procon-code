use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut base = "indeednow".to_string().chars().collect::<Vec<_>>();
    base.sort();

    for i in 0..n {
        let mut text = s[i].clone();
        text.sort();

        if text == base {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
