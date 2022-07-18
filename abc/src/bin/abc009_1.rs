use num_integer::Integer;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", Integer::div_ceil(&n, &2));
}
