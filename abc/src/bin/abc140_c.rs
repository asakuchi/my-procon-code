use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n - 1],
    }

    let mut a = vec![0; n];

    a[0] = b[0];
    a[n - 1] = b[n - 2];

    for i in 1..n - 1 {
        a[i] = b[i].min(b[i - 1]);
    }

    let result: usize = a.iter().sum();

    println!("{}", result);
}
