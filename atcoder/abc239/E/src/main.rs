use proconio::fastout;
use proconio::input;

const K_MAX: usize = 20;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize; n],
        mut ab: [(usize, usize); n-1],
        mut vk: [(usize, usize); q],
    }

    let mut matrix = vec![Vec::with_capacity(n); n];

    for &(a, b) in ab.iter() {
        matrix[a - 1].push(b - 1);
        matrix[b - 1].push(a - 1);
    }

    let mut result = vec![Vec::with_capacity(n); n];

    dfs(&matrix, &x, &mut result, 0, 0);

    // println!("{:?}", result);

    for &(v, k) in vk.iter() {
        println!("{}", result[v - 1][k - 1]);
    }
}

fn dfs(
    matrix: &Vec<Vec<usize>>,
    x: &Vec<usize>,
    result: &mut Vec<Vec<usize>>,
    parent: usize,
    current: usize,
) -> Vec<usize> {
    let mut numbers = vec![x[current]];

    for &child in matrix[current].iter() {
        if child == parent {
            continue;
        }

        let childs_top = dfs(matrix, x, result, current, child);

        numbers.append(&mut childs_top.clone());
    }

    numbers.sort_by_key(|&value| std::cmp::Reverse(value));
    let top_k = if numbers.len() > K_MAX {
        numbers[0..=K_MAX].to_vec()
    } else {
        numbers
    };

    result[current] = top_k.clone();

    top_k
}
