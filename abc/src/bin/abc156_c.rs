use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [isize; n],
    }

    let &min_p = x.iter().min().unwrap();
    let &max_p = x.iter().max().unwrap();

    let mut result = 1_000_000_000;

    for p in min_p..=max_p {
        let mut score = 0;

        for i in 0..n {
            let x_i = x[i];

            score += (p - x_i).pow(2);
        }

        result = result.min(score);
    }

    println!("{}", result);
}
