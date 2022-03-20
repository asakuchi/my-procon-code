use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const MODULO: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        s: Usize1,
        t: Usize1,
        x: Usize1,
        mut uv: [(Usize1, Usize1); m],
    }

    let mut matrix = vec![vec![false; n]; n];

    for &(u, v) in uv.iter() {
        matrix[u][v] = true;
        matrix[v][u] = true;
    }

    let mut memo = vec![vec![vec![0; 2]; k + 1]; n];

    memo[s][0][0] = 1;

    for i in 0..k {
        for &(u, v) in uv.iter() {
            for y in 0..2 {
                memo[v][i + 1][if v == x { (!(y != 0)) as usize } else { y }] += memo[u][i][y];
                memo[u][i + 1][if u == x { (!(y != 0)) as usize } else { y }] += memo[v][i][y];

                memo[v][i + 1][0] %= MODULO;
                memo[v][i + 1][1] %= MODULO;
                memo[u][i + 1][0] %= MODULO;
                memo[u][i + 1][1] %= MODULO;
            }
        }
    }

    println!("{}", memo[t][k][0]);

    // let mut memo = vec![vec![vec![-1; 2]; k + 1]; n];

    // let result = rec(&matrix, n, m, t, x, s, k, true, &mut memo);

    // println!("{}", result % MODULO);
}

// fn rec(
//     matrix: &Vec<Vec<bool>>,
//     n: usize,
//     m: usize,
//     t: usize,
//     x: usize,
//     current: usize,
//     rest: usize,
//     is_x_even: bool,
//     memo: &mut Vec<Vec<Vec<isize>>>,
// ) -> usize {
//     if rest == 0 {
//         if current == t && is_x_even {
//             return 1;
//         } else {
//             return 0;
//         }
//     }

//     if memo[current][rest][is_x_even as usize] != -1 {
//         return memo[current][rest][is_x_even as usize] as usize;
//     }

//     let mut result = 0;

//     let is_x_even_next = if current == x { !is_x_even } else { is_x_even };

//     for next in 0..n {
//         if matrix[current][next] {
//             result += rec(matrix, n, m, t, x, next, rest - 1, is_x_even_next, memo);
//             result %= MODULO;
//         }
//     }

//     result %= MODULO;

//     memo[current][rest][is_x_even as usize] = result as isize;

//     result
// }
