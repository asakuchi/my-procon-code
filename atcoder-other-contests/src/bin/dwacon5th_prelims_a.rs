use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let sum: isize = a.iter().sum();

    let mut result = 0;
    let mut min_diff = 10000;

    for i in 0..n {
        let value = a[i];

        let diff = (value * n as isize - sum).abs();

        if min_diff > diff {
            min_diff = diff;
            result = i;
        }
    }

    println!("{}", result);
}
