use ::procon_library_rs::binary_indexed_tree::BinaryIndexedTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut bit = BinaryIndexedTree::new(n + 1);

    let mut result = 0;

    for x in a {
        result += bit.sum(n) - bit.sum(x);

        bit.add(x, 1);
    }

    println!("{}", result);
}
