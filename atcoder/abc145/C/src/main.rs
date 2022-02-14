use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(isize, isize); n],
    }

    let mut sum = 0.;
    let mut count = 0;

    for permutate in (0..n).permutations(n) {
        for i in 1..n {
            let distance_pow = (xy[permutate[i]].0 - xy[permutate[i - 1]].0).pow(2)
                + (xy[permutate[i]].1 - xy[permutate[i - 1]].1).pow(2);
            let distance = (distance_pow as f64).sqrt();

            sum += distance;
        }
        count += 1;
    }

    println!("{}", sum / count as f64);
}
