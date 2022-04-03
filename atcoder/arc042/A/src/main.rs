use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    a.reverse();

    let mut list = Vec::with_capacity(n);
    let mut set = std::collections::HashSet::with_capacity(n);

    for &value in a.iter() {
        if !set.contains(&value) {
            list.push(value);
        }

        set.insert(value);
    }

    for value in 1..=n {
        if !set.contains(&value) {
            list.push(value);
        }
    }

    for value in list {
        println!("{}", value);
    }
}
