use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: [char; 3],
        t: [char; 3],
    }

    let mut same = 0;

    for i in 0..3 {
        if s[i] == t[i] {
            same += 1;
        }
    }

    println!("{}", if same != 1 { "Yes" } else { "No" });
}
