use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let nums: Vec<usize> = s.split('.').map(|t| t.parse().unwrap()).collect();

    let x = nums[0];
    let y = nums[1];

    println!(
        "{}{}",
        x,
        if y <= 2 {
            "-"
        } else if 3 <= y && y <= 6 {
            ""
        } else {
            "+"
        }
    );
}
