use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        m: usize,
        n: usize,
        matrix: [[usize; m]; n],
    }

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            let mut visited = vec![vec![false; m]; n];
            let count = rec(&matrix, &mut visited, n, m, i as isize, j as isize);

            result = std::cmp::max(result, count);
        }
    }

    println!("{}", result);
}

fn rec(
    matrix: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    n: usize,
    m: usize,
    i: isize,
    j: isize,
) -> usize {
    if i < 0 || i >= n as isize || j < 0 || j >= m as isize {
        return 0;
    }

    if matrix[i as usize][j as usize] == 0 {
        return 0;
    }

    if visited[i as usize][j as usize] {
        return 0;
    }

    visited[i as usize][j as usize] = true;
    let mut result = 0;

    let direction = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    for &(x, y) in direction.iter() {
        let next_x = i + x;
        let next_y = j + y;

        let count = rec(matrix, visited, n, m, next_x, next_y);

        result = std::cmp::max(result, count);
    }

    result += 1; // 自分自身

    visited[i as usize][j as usize] = false;

    result
}
