use num_integer::*;
use proconio::input;

fn main() {
    input! {
        mut n: usize,
        m: usize,
        mut ac: [(usize, usize); m],
    }

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    ac.sort_by_key(|x| x.1);

    for i in 0..m {
        let (a, c) = ac[i];

        let g = n.gcd(&a);

        total_cost += (n - g) * c;

        n = g;
    }

    if n == 1 {
        println!("{}", total_cost);
    } else {
        println!("-1");
    }
}
