use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(f64, f64); n],
    }

    let time: Vec<_> = ab.iter().map(|&(a, b)| a / b).collect();

    let mut total_time = 0.;

    for value in time.iter() {
        total_time += value;
    }

    let mut collision_time = total_time / 2.;
    let mut result = 0.;

    for i in 0..n {
        if time[i] < collision_time {
            collision_time -= time[i];
            result += ab[i].0;
        } else {
            result += ab[i].1 * collision_time;
            break;
        }
    }

    println!("{}", result);
}
