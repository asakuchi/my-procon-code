use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; n * 5],
    }

    x.sort();

    let mut sum = 0;

    for i in n..4 * n {
        sum += x[i];
    }

    println!("{}", sum as f64 / (3. * n as f64));
}
