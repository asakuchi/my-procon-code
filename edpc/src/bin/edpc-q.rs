use proconio::input;

use ac_library_rs::{Max, Segtree};

fn main() {
    input! {
        n: usize,
        h: [usize; n],
        a: [usize; n],
    }

    let mut tree = Segtree::<Max<usize>>::new(n + 1);

    for i in 0..n {
        let beauty = a[i];
        let height = h[i];

        let prev = tree.prod(0, height);
        let old = tree.prod(0, height + 1);

        tree.set(height, old.max(prev + beauty));
    }

    println!("{}", tree.prod(0, n + 1));
}
