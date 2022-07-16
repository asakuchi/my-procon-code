use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    a.sort();

    let &max = a.iter().max().unwrap();

    let mut result = a[0];

    for &num in a.iter() {
        if (max as f64 / 2. - num as f64).abs() < (max as f64 / 2. - result as f64).abs() {
            result = num;
        }
    }

    println!("{} {}", max, result);
}
