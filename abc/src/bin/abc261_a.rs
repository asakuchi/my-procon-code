use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        l_1: usize,
        r_1: usize,
        l_2: usize,
        r_2: usize,
    }

    if r_1.min(r_2) >= l_1.max(l_2) {
        let result = r_1.min(r_2) - l_1.max(l_2);
        println!("{}", result);
    } else {
        println!("0");
    }
}
