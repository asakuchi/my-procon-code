use itertools::Itertools;
use num_integer::Roots;
use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut result = vec![vec![-1; n]; n];
    let mut visited = vec![vec![false; n]; n];

    let mut queue = VecDeque::new();
    queue.push_back((0 as usize, 0 as usize));
    result[0][0] = 0;
    visited[0][0] = true;

    while let Some((i, j)) = queue.pop_front() {
        for k in 0..n {
            let i_k_diff = (i as isize - k as isize).pow(2);

            if (m as isize) < i_k_diff {
                continue;
            }

            let sqrt = (m as isize - i_k_diff).sqrt();

            if sqrt * sqrt != (m as isize - i_k_diff) {
                continue;
            }

            for l in vec![j as isize + sqrt, j as isize - sqrt] {
                if l < 0 {
                    continue;
                }

                let l = l as usize;

                if l >= n {
                    continue;
                }

                if visited[k][l] {
                    continue;
                }

                let distance = (i as isize - k as isize).pow(2) + (j as isize - l as isize).pow(2);

                if distance == (m as isize) {
                    result[k][l] = result[i][j] + 1;
                    visited[k][l] = true;
                    queue.push_back((k as usize, l as usize));
                }
            }
        }
    }

    for i in 0..n {
        let text = result[i].iter().join(" ");
        println!("{}", text);
    }
}
