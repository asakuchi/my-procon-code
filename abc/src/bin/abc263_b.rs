use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n-1],
    }

    let mut current = n;

    let mut result = 0;

    while current != 1 {
        current = p[current - 2];
        result += 1;
    }

    println!("{}", result);
}
