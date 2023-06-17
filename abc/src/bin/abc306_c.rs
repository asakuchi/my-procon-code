use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n * 3],
    }

    let mut list = vec![Vec::new(); n + 1];

    for i in 0..n * 3 {
        list[a[i]].push(i + 1);
    }

    let mut result = Vec::new();

    for i in 1..=n {
        result.push((i, list[i][1]));
    }

    result.sort_by_key(|x| x.1);

    let text = result.iter().map(|x| x.0).format(" ");

    println!("{}", text);
}
