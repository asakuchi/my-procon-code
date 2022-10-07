use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut set = FxHashSet::default();

    for value in s {
        set.insert(value);
    }

    println!("{}", set.len());
}
