use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut set = std::collections::HashSet::new();

    for i in a {
        set.insert(i);
    }

    println!("{}", set.len());
}
