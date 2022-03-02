use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let mut set = std::collections::HashSet::new();

    for a in 1..=1000 {
        for b in 1..=1000 {
            let area = 3 * a + 2 * b * 2 * a + 3 * b;

            set.insert(area);
        }
    }

    let mut count = 0;

    for area in s.iter() {
        if !set.contains(area) {
            count += 1;
        }
    }

    println!("{}", count);
}
