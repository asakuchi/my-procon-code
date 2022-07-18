use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    let list = vec![a, b, c];

    let mut list = list.iter().enumerate().collect::<Vec<_>>();

    list.sort_by_key(|x| std::cmp::Reverse(x.1));

    let mut list = list
        .iter()
        .enumerate()
        .map(|(i, x)| (x.0, i))
        .collect::<Vec<_>>();

    list.sort_by_key(|x| x.0);

    for i in 0..3 {
        println!("{}", list[i].1 + 1);
    }
}
