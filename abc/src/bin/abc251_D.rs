use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        _w: usize,
    }

    let mut list = Vec::new();

    for c in 1..100 {
        list.push(c);
    }

    for b in 1..100 {
        list.push(b * 100usize.pow(1));
    }

    for a in 1..100 {
        list.push(a * 100usize.pow(2));
    }

    list.push(1_000_000);

    println!("{}", list.len());

    let a: String = list
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", a);
}
