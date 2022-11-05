use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.prev_permutation();

    println!("{}", a.iter().format(" "));
}
