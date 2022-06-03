use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut sp: [(String, usize); n],
    }

    let mut list: Vec<_> = sp.iter().enumerate().collect();

    list.sort_by_key(|x| ((x.1).0.clone(), std::cmp::Reverse((x.1).1)));

    for (i, (_name, _score)) in list {
        println!("{}", i + 1);
    }
}
