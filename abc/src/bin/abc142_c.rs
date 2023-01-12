use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut list: Vec<_> = a.iter().enumerate().collect();

    list.sort_by_key(|&(_i, &value)| value);

    let text = list.iter().map(|&(i, _value)| i + 1).format(" ");

    println!("{}", text);
}
