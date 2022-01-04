// -*- coding:utf-8-unix -*-

use proconio::input;

#[derive(Copy, Clone, Debug)]
enum Cell {
    /// 未踏
    Unexplored,
    Step(usize),
    /// 通行不可
    Unavailable,
}

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

    let mut steps = vec![vec![Cell::Unexplored; width]; height];
    steps[start_row][start_col] = Cell::Step(0);
    // println!("steps:{:?}", steps);
    let mut queue = std::collections::VecDeque::<(usize, usize)>::new();
    queue.push_back((start_row, start_col));

    while let Some(target) = queue.pop_front() {
        for pattern in move_row.iter().zip(move_col.iter()) {
            let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

            if 0 > next.0 || next.0 >= height as isize || 0 > next.1 || next.1 >= width as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if maze[next.0].chars().nth(next.1).unwrap() == 'X' {
                steps[next.0][next.1] = Cell::Unavailable;
                continue;
            }

            if let Cell::Unexplored = steps[next.0][next.1] {
                if let Cell::Step(step) = steps[target.0][target.1] {
                    steps[next.0][next.1] = Cell::Step(step + 1);
                    queue.push_back(next);
                }
            }
        }
    }

    let mut result = 0;
    let mut is_unreachable = false;

    // println!("steps:{:?}", steps);

    'outer: for line in steps.iter() {
        for &value in line.iter() {
            match value {
                Cell::Unexplored => {
                    is_unreachable = true;
                    break 'outer;
                }
                Cell::Step(step) => {
                    result = std::cmp::max(step, result);
                }
                Cell::Unavailable => {
                    // do nothing.
                }
            }
        }
    }

    if is_unreachable {
        println!("{}", -1);
    } else {
        println!("{}", result);
    }
}
