use proconio::input;

use ::ac_library_rs::scc::SccGraph;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(usize, usize); m],
    }

    let mut g = SccGraph::new(n);

    for &(a, b) in &a_b {
        g.add_edge(a, b);
    }

    let scc = g.scc();

    println!("{}", scc.len());

    for item in scc {
        print!("{}", item.len());

        for x in item {
            print!(" {}", x);
        }

        println!();
    }
}
