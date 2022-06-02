use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut x: [isize; m],
    }

    x.sort();

    let mut diff = Vec::new();

    for i in 1..m {
        diff.push(x[i] - x[i - 1]);
    }

    diff.sort();

    for _ in 0..n - 1 {
        diff.pop();
    }

    println!("{}", diff.iter().sum::<isize>());
}
