use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [[usize]; n],
        mut s_t: [(Usize1, Usize1); q],
    }

    for (s, t) in s_t {
        println!("{}", a[s][t]);
    }
}
