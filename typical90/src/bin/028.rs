//!
//! 二次元いもす法
//!

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize, usize, usize); n],
    }

    let mut table = vec![vec![0isize; 1_001]; 1_001];

    for &(l_x, l_y, r_x, r_y) in lr.iter() {
        table[l_x][l_y] += 1;
        table[r_x][l_y] -= 1;
        table[l_x][r_y] -= 1;
        table[r_x][r_y] += 1;
    }

    for x in 0..=1_000 {
        for y in 1..=1_000 {
            table[x][y] += table[x][y - 1];
        }
    }

    for x in 1..=1_000 {
        for y in 0..=1_000 {
            table[x][y] += table[x - 1][y];
        }
    }

    let mut answer = vec![0usize; n + 1];

    for x in 0..=1_000 {
        for y in 0..=1_000 {
            answer[table[x][y] as usize] += 1;
        }
    }

    for number in &answer[1..] {
        println!("{}", number);
    }
}
