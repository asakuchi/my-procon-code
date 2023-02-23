use ac_library_rs::FenwickTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut bit = FenwickTree::new(n + 1, 0isize);

    let mut result = 0;

    for x in a {
        result += bit.sum(x, n + 1);

        bit.add(x, 1);
    }

    println!("{}", result);
}
