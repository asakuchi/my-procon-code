use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize; n],
        b: [isize; n],
    }

    let diff: isize = a
        .iter()
        .zip(b.iter())
        .map(|(a_value, b_value)| (a_value - b_value).abs())
        .sum();

    println!(
        "{}",
        if (k - diff) >= 0 && (k - diff) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
