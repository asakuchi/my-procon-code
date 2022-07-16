use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut st: [(String, usize); n],
    }

    let mut result = 0;
    let mut max_score = 0;

    let mut set = std::collections::HashSet::new();

    for i in 0..n {
        if !set.contains(&st[i].0) {
            set.insert(st[i].0.clone());

            if max_score < st[i].1 {
                result = i;
                max_score = st[i].1;
            }
        }
    }

    println!("{}", result + 1);
}
