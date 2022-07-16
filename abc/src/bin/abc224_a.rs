use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    if &s[s.len() - 2..s.len()] == "er" {
        println!("er");
    } else {
        println!("ist");
    }
}
