use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut q = vec![0; n];

    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }

    for num in q {
        println!("{}", num);
    }
}
