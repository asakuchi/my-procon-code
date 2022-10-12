use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        mut a: [usize; n],
    }

    let mut max_diff = k - a[a.len() - 1] + a[0];

    for i in 1..n {
        let diff = a[i] - a[i - 1];

        max_diff = max_diff.max(diff);
    }

    println!("{}", k - max_diff);
}
