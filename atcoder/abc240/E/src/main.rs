use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut uv: [(usize, usize); n-1],
    }

    let mut matrix = vec![Vec::with_capacity(n); n];

    for &(u, v) in uv.iter() {
        matrix[u - 1].push(v - 1);
        matrix[v - 1].push(u - 1);
    }

    let mut result = vec![(0, 0); n];

    rec(&matrix, &mut result, -1, 0, &mut 1);

    for i in 0..n {
        println!("{} {}", result[i].0, result[i].1);
    }
}

fn rec(
    matrix: &Vec<Vec<usize>>,
    result: &mut Vec<(usize, usize)>,
    prev: isize,
    current: usize,
    leaf: &mut usize,
) {
    let mut child_count = 0;
    result[current] = (*leaf, *leaf);

    // println!("node:{} {:?}", current, result[current]);

    for &next in matrix[current].iter() {
        if next as isize == prev {
            continue;
        }

        rec(matrix, result, current as isize, next, leaf);
        result[current].1 = std::cmp::max(result[current].1, result[next].1);

        // println!("update node:{} {:?}", current, result[current]);

        child_count += 1;
    }

    if child_count == 0 {
        *leaf += 1;
    }
}
