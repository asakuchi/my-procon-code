use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,

    }

    let mut list = vec![a, b, c];
    list.sort();

    if list[1] == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
