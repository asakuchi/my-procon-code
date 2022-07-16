use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut a = n;

    for _ in 1..=k {
        let f = g1(a) - g2(a);
        a = f;
    }

    println!("{}", a);
}

fn g1(x: usize) -> usize {
    let text = x.to_string();

    let mut list: Vec<_> = text.split("").collect();
    list.sort_by_key(|&num| std::cmp::Reverse(num));

    let text = list.join("");

    text.parse().unwrap()
}

fn g2(x: usize) -> usize {
    let text = x.to_string();

    let mut list: Vec<_> = text.split("").collect();
    list.sort();

    let text = list.join("");

    text.parse().unwrap()
}
