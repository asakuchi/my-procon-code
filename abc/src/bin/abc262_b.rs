use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut u_v: [(Usize1, Usize1); m],
    }

    let mut matrix = vec![vec![false; n]; n];

    for &(u, v) in &u_v {
        matrix[u][v] = true;
        matrix[v][u] = true;
    }

    let mut result = 0;

    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if matrix[a][b] && matrix[b][c] && matrix[c][a] {
                    result += 1;
                }
            }
        }
    }

    println!("{}", result);
}
