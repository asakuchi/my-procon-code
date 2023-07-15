use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        d: [usize; n],
    }

    let mut result = p;

    for i in 0..n {
        result = result.min(d[i] + q);
    }

    println!("{}", result);
}
