use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut prev = a[0];

    for &value in &a {
        if value < prev {
            break;
        }

        prev = value;
    }

    // prev を消す
    let result = a.iter().filter(|&&x| x != prev).format(" ");

    println!("{}", result);
}
