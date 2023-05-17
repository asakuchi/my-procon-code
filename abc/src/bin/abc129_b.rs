use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [isize; n],
    }

    let mut result = 1_000_000;

    for t in 0..n {
        let mut s_1 = Vec::new();
        let mut s_2 = Vec::new();

        for i in 0..=t {
            s_1.push(w[i]);
        }

        for i in t + 1..n {
            s_2.push(w[i]);
        }

        let sum_1: isize = s_1.into_iter().sum();
        let sum_2: isize = s_2.into_iter().sum();

        result = result.min((sum_1 - sum_2).abs());
    }

    println!("{}", result);
}
