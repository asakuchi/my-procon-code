use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_p_x: [(usize, usize,usize); n],
    }

    let result = a_p_x
        .iter()
        .filter(|(a, _, x)| *a < *x)
        .map(|&(_a, p, _x)| p)
        .sorted()
        .next();

    if let Some(price) = result {
        println!("{}", price);
    } else {
        println!("-1");
    }
}
