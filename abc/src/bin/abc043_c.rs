use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut result = 1_000_000_000_000;

    for x in -100..=100 {
        let mut score = 0;

        for i in 0..n {
            score += (x - a[i]).pow(2);
        }

        result = result.min(score);
    }

    println!("{}", result);
}
