use proconio::{input, marker::Usize1};

use ac_library_rs::scc::SccGraph;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }

    let mut result = 0;

    let mut g = SccGraph::new(n);

    for i in 0..n {
        if i == a[i] {
            result += 1;
        }

        g.add_edge(i, a[i]);
    }

    let scc = g.scc();

    for list in scc.iter() {
        if list.len() >= 2 {
            result += list.len();
        }
    }

    println!("{}", result);
}
