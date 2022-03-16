use proconio::input;

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        mut x: usize,
    }

    let sum: usize = a.iter().sum();

    let mut result = 0;

    if x > sum {
        result = (x / sum) * n;
        x %= sum;
    }

    let mut total = 0;

    for i in 0..n {
        total += a[i];

        if total > x {
            println!("{}", result + i + 1);
            return;
        }
    }

    panic!("k not found");
}
