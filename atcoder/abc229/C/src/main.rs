use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut w: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_by_key(|x| std::cmp::Reverse(x.0));

    let mut result = 0;

    for (a, b) in ab {
        if w >= b {
            w -= b;
            result += a * b;
        } else {
            result += a * w;
            break;
        }
    }

    println!("{}", result);
}
