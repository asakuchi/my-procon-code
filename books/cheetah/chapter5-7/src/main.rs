// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize, // maze の要素数
        maze: [String; n], // 各文字は '.' か 'X'
        start_row: usize,
        start_col: usize,
        move_size: usize, // move_row,move_colの要素数
        move_row: [isize; move_size],
        move_col: [isize; move_size],
    }

    // println!("start_row:{}", start_row);
    // println!("start_col:{}", start_col);
    // println!("maze:{:?}", maze);
    // println!("move_row:{:?}", move_row);
    // println!("move_col:{:?}", move_col);

    let width = maze[0].len();
    let height = n;

    let mut steps = vec![vec![-1; width]; height];
    steps[start_row][start_col] = 0;

    // println!("steps:{:?}", steps);

    let mut queue = std::collections::VecDeque::<(isize, isize)>::new();
    queue.push_back((start_row as isize, start_col as isize));

    while queue.len() > 0 {
        let target = queue.pop_front().unwrap();

        for pattern in move_row.iter().zip(move_col.iter()) {
            let next = (target.0 + pattern.0, target.1 + pattern.1);

            if 0 <= next.0
                && next.0 < height as isize
                && 0 <= next.1
                && next.1 < width as isize
                && steps[next.0 as usize][next.1 as usize] == -1
                && maze[next.0 as usize].chars().nth(next.1 as usize).unwrap() != 'X'
            {
                steps[next.0 as usize][next.1 as usize] =
                    steps[target.0 as usize][target.1 as usize] + 1;
                queue.push_back(next);
            }
        }
    }

    let mut result = 0;

    // println!("steps:{:?}", steps);

    'outer: for (i, line) in steps.iter().enumerate() {
        for (j, &value) in line.iter().enumerate() {
            if maze[i].chars().nth(j).unwrap() != 'X' && value == -1 {
                result = -1;
                break 'outer;
            }

            result = std::cmp::max(value, result);
        }
    }

    println!("{}", result);
}
