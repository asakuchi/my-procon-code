// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        l: [[usize]; n],
    }
    //std::usize::MAX
    // let mut memo = vec![vec![0; 1000000000000000000]; n];
    let sum = dfs(n, x, &l, 0, 1);
    println!("{}", sum);
}

fn dfs(
    n: usize,
    x: usize,
    l: &Vec<Vec<usize>>,
    index: usize,
    product: usize,
    // memo: &mut Vec<Vec<usize>>,
) -> usize {
    if index == n {
        if product == x {
            return 1;
        } else {
            return 0;
        }
    }

    if product > x {
        return 0;
    }

    // if memo[index][product] != 0 {
    //     return memo[index][product] as usize;
    // }

    let mut sum = 0;

    for ball in l[index].iter() {
        let new_product = product.checked_mul(*ball);

        match new_product {
            Some(n_p) => {
                sum += dfs(n, x, l, index + 1, n_p);
            }
            None => {
                sum += 0;
            }
        }
    }

    // memo[index][product] = sum as i64;

    return sum;
}
