use ac_library_rs::floor_sum;
use proconio::input;

fn main() {
    input! {
        t: usize,
        n_m_a_b: [(i64, i64, i64, i64); t],
    }

    for (n, m, a, b) in n_m_a_b {
        println!("{}", floor_sum(n, m, a, b));
    }
}
