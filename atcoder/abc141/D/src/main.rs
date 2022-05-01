use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut list = std::collections::BinaryHeap::from(a);

    for _ in 0..m {
        let price = list.pop().unwrap();
        list.push(price / 2);
    }

    let result: usize = list.iter().sum();

    println!("{}", result);
}
