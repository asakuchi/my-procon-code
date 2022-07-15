use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mut ab: [(usize, usize); n],
    }

    let mut total_time = 0;

    let mut min = 0;

    for i in 0..n {
        let (a, b) = ab[i];

        total_time += a + b;

        let time = total_time + (x - i - 1) * b;

        if min == 0 {
            min = time;
        } else {
            min = min.min(time);
        }
    }

    println!("{:?}", min);
}
