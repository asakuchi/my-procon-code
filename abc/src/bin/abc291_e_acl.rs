use ac_library_rs::SccGraph;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x_y: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    let mut g = SccGraph::new(n);

    for &(x, y) in &x_y {
        list[x].push(y);

        g.add_edge(x, y);
    }

    let scc = g.scc();
    let sorted: Vec<_> = scc.into_iter().flatten().collect();

    if sorted.len() != n {
        // これは入力に矛盾するAが存在するということで、
        // それは制約で入力されないということになっている
        println!("No");
        return;
    }

    // validate
    for i in 1..n {
        // ソートした時に隣あった要素同士にパスがなければならない
        if !list[sorted[i - 1]].contains(&sorted[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");

    let mut result = vec![0; n];
    let mut num = 1;

    for i in sorted {
        result[i] = num;
        num += 1;
    }

    let text = result.iter().join(" ");

    println!("{}", text);
}
