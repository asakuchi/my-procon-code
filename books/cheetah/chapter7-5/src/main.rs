// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第7章 キングとナイト
///
fn main() {
    input! {
        size: usize,
        start: (isize, isize),
        end: (isize, isize),
        num_moves: usize,
    }

    let hands: Vec<(isize, isize)> = vec![
        // king
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        // knight
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
        (-2, 1),
        (-1, 2),
        (2, 1),
        (1, 2),
    ];

    let mut memo = vec![vec![vec![-1 as i64; size as usize]; size]; num_moves + 1];

    let sum = dfs(size, &hands, end, start, num_moves, &mut memo);

    println!("{}", sum);
}

fn dfs(
    size: usize,
    hands: &Vec<(isize, isize)>,
    end: (isize, isize),
    now: (isize, isize),
    remaining_move: usize,
    memo: &mut Vec<Vec<Vec<i64>>>,
) -> usize {
    if remaining_move == 0 {
        if now.0 == end.0 && now.1 == end.1 {
            return 1;
        } else {
            return 0;
        }
    }

    if memo[remaining_move][now.0 as usize][now.1 as usize] != -1 {
        return memo[remaining_move][now.0 as usize][now.1 as usize] as usize;
    }

    let mut sum = 0;

    for hand in hands {
        let next = (now.0 + hand.0, now.1 + hand.1);

        if next.0 < 0 || next.1 < 0 || next.0 >= size as isize || next.1 >= size as isize {
            continue;
        }

        sum += dfs(size, hands, end, next, remaining_move - 1, memo);
    }

    memo[remaining_move][now.0 as usize][now.1 as usize] = sum as i64;

    sum
}
