use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut s: String,
    }

    let mut min_str = s.clone();
    let mut max_str = s.clone();

    for _ in 0..s.len() {
        // println!("{}", s);
        s = format!("{}{}", &s[1..s.len()], &s[0..1]);

        if s < min_str {
            min_str = s.clone();
        }

        if s > max_str {
            max_str = s.clone();
        }
    }

    println!("{}", min_str);
    println!("{}", max_str);
}
