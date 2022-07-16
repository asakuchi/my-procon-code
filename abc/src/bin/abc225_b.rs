use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(Usize1, Usize1); n-1],
    }

    let mut count = vec![0; n];

    for &(a, b) in ab.iter() {
        count[a] += 1;
        count[b] += 1;
    }

    for &value in count.iter() {
        if value == n - 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
