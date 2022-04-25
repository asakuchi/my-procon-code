///
/// 順列全探索
///
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    }

    let mut matrix = vec![vec![false; n]; n];

    for &(a, b) in ab.iter() {
        matrix[a][b] = true;
        matrix[b][a] = true;
    }

    let mut result = 0;

    'outer: for mut permutation in (1..n).permutations(n - 1) {
        let mut nodes = vec![0];
        nodes.append(&mut permutation);

        for i in 1..n {
            if !matrix[nodes[i]][nodes[i - 1]] {
                continue 'outer;
            }
        }

        result += 1;
    }

    println!("{}", result);
}
