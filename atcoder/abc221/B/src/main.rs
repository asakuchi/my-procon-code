use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    if s == t {
        println!("Yes");
        return;
    }

    for i in 1..s.len() {
        if format!(
            "{}{}{}",
            &s[..i - 1],
            &s[i - 1..i + 1].chars().rev().collect::<String>(),
            &s[i + 1..]
        ) == t
        {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
