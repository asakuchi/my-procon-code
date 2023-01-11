use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        start: isize,
        x: [isize; n],
    }

    let list: Vec<_> = x.iter().map(|&value| (value - start).abs()).collect();

    // println!("{:?}", list);

    let mut gcd = list[0];

    for i in 1..n {
        gcd = gcd.gcd(&list[i]);
    }

    println!("{}", gcd);
}
