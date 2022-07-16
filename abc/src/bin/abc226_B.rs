use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [[usize]; n],
    }

    let mut set = std::collections::HashSet::new();

    for record in a.iter() {
        set.insert(record);
    }

    println!("{}", set.len());
}
