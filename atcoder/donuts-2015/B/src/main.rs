use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        mut bi: [(usize, [Usize1]); m],
    }

    let mut result = 0;

    for combination in (0..n).combinations(9) {
        let mut score = 0;

        for &member in &combination {
            score += a[member];
        }

        for i in 0..m {
            let mut count = 0;

            for &combo_member in &bi[i].1 {
                if combination.contains(&combo_member) {
                    count += 1;
                }
            }

            if count >= 3 {
                score += bi[i].0;
            }
        }

        result = result.max(score);
    }

    println!("{}", result);
}
