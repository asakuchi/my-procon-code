use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        u_v: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut used = 0_usize;

    let mut result = Vec::new();

    for i in 0..n {
        if list[i].len() >= 3 {
            result.push(list[i].len());

            used += list[i].len() + 1;
        }
    }

    let rest = n - used;

    for _ in 0..rest / 3 {
        result.push(2);
    }

    result.sort();

    let text = result.iter().format(" ");

    println!("{}", text);
}
