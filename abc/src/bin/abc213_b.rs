use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut list = a
        .iter()
        .enumerate()
        .map(|(i, &value)| (value, i + 1))
        .collect::<Vec<_>>();

    list.sort();

    println!("{}", list[list.len() - 2].1);
}
