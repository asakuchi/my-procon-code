use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for i in 0..n {
        result += a[i] - 1;
    }

    println!("{}", result);
}
