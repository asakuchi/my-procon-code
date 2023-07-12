// 解説AC

use proconio::{input, marker::Usize1};

use ac_library_rs::ModInt1000000007 as mint;

fn main() {
    input! {
        n: usize,
        u_v_w: [(Usize1, Usize1, usize); n-1],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v, w) in &u_v_w {
        list[u].push((v, w));
        list[v].push((u, w));
    }

    let mut result = mint::from(0);

    let mut dist = vec![0; n];
    rec(n, &list, 0, 0, 0, &mut dist);

    for bit in 0..60 {
        let mut one = 0;
        let mut zero = 0;

        for i in 0..n {
            if dist[i] & 1 << bit > 0 {
                one += 1;
            } else {
                zero += 1;
            }
        }

        result += mint::from(one) * mint::from(zero) * mint::from(1_usize << bit);
    }

    println!("{}", result);
}

fn rec(
    n: usize,
    list: &Vec<Vec<(usize, usize)>>,
    current: usize,
    parent: usize,
    total: usize,
    dist: &mut Vec<usize>,
) {
    dist[current] = total;

    for &(next, weight) in list[current].iter() {
        if next == parent {
            continue;
        }

        rec(n, list, next, current, total ^ weight, dist);
    }
}
