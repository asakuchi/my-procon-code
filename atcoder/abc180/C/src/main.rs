use num_integer::*;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut list = Vec::new();

    for i in 1..=(n + 1).sqrt() {
        if n % i == 0 {
            list.push(i);
            list.push(n / i);
        }
    }

    list.sort();
    list.dedup();

    for num in list {
        println!("{}", num);
    }
}
