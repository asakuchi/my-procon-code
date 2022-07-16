use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        a: [usize; n],
    }

    let mut set = std::collections::HashSet::new();

    for select in 1..=3 {
        for combination in (0..n).combinations(select) {
            let mut sum = 0;

            for i in combination {
                sum += a[i];
            }

            if sum <= w {
                set.insert(sum);
            }
        }
    }

    println!("{}", set.len());
}
