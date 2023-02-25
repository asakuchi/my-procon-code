// for interactive
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
// println!("{}", x);
// stdout().flush().unwrap();

// use rand::prelude::*;
use rand::seq::SliceRandom;
// use rand::Rng;

// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;
// use proconio::marker::Usize1;

// use std::collections::BinaryHeap;
use std::time::Instant;

use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug)]
enum ExcavationResult {
    /// 指定したセルの岩盤が破壊できなかった
    NotBreak,
    /// 指定したセルの岩盤が破壊できた（家が存在するセルでまだ水が流れていないものがある）
    Broken,
    /// 指定したセルの岩盤が破壊できた（家が存在する全てのセルに水が流れている）
    Complete,
    /// 不正な出力
    Illegal,
}

#[derive(Copy, Clone, Debug)]
enum CellStatus {
    Normal,
    Broken,
}

#[derive(Copy, Clone, Debug)]
enum CellType {
    Water,
    House,
    Else,
}

#[derive(Clone, Debug)]
struct CellData {
    status: CellStatus,
    cell_type: CellType,
    estimated_hit_point: usize,
}

impl CellData {
    fn new() -> CellData {
        CellData {
            status: CellStatus::Normal,
            cell_type: CellType::Else,
            estimated_hit_point: 5_000,
        }
    }
}

fn main() {
    let start = Instant::now();

    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        n: usize,
        w: usize,
        k: usize,
        c: usize,
        water_source: [(usize, usize); w],
        house: [(usize, usize); k],
    }

    let mut rng = rand::thread_rng();

    let mut grid = vec![vec![CellData::new(); n]; n];

    for &(i, j) in &water_source {
        grid[i][j].cell_type = CellType::Water;
    }

    for &(i, j) in &house {
        grid[i][j].cell_type = CellType::House;
    }

    // とりあえず水源は全て壊す必要がある
    for i in 0..w {
        let (y, x) = water_source[i];

        // eprintln!("water {:?}", (y, x));

        loop {
            // 壊れるまでやる

            let power = (c * 3).min(grid[y][x].estimated_hit_point);
            grid[y][x].estimated_hit_point -= power;

            let replay = query(y, x, power, &mut source);

            match replay {
                ExcavationResult::Complete | ExcavationResult::Illegal => return,
                ExcavationResult::Broken => {
                    grid[y][x].status = CellStatus::Broken;
                    grid[y][x].estimated_hit_point = 0;
                    break;
                }
                ExcavationResult::NotBreak => {}
            }
        }
    }

    // 家も全て壊す必要がある
    for i in 0..k {
        let (y, x) = house[i];

        // eprintln!("house {:?}", (y, x));

        loop {
            // 壊れるまでやる

            let power = (c * 3).min(grid[y][x].estimated_hit_point);
            grid[y][x].estimated_hit_point -= power;

            let replay = query(y, x, power, &mut source);

            match replay {
                ExcavationResult::Complete | ExcavationResult::Illegal => return,
                ExcavationResult::Broken => {
                    grid[y][x].status = CellStatus::Broken;
                    grid[y][x].estimated_hit_point = 0;
                    break;
                }
                ExcavationResult::NotBreak => {}
            }
        }
    }

    let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = BinaryHeap::new();

    // 水源に近い方から叩く
    for i in 0..w {
        let (y, x) = water_source[i];

        directions.shuffle(&mut rng);

        for &direction in &directions {
            let next_y = y as isize + direction.0;
            let next_x = x as isize + direction.1;

            if next_y < 0 || next_y >= n as isize || next_x < 0 || next_x >= n as isize {
                continue;
            }

            let next_y = next_y as usize;
            let next_x = next_x as usize;

            if let CellStatus::Broken = grid[next_y][next_x].status {
                continue;
            }

            queue.push((0, next_y, next_x));
        }
    }

    // 家に近い方からも叩く
    for i in 0..w {
        let (y, x) = house[i];

        directions.shuffle(&mut rng);

        for &direction in &directions {
            let next_y = y as isize + direction.0;
            let next_x = x as isize + direction.1;

            if next_y < 0 || next_y >= n as isize || next_x < 0 || next_x >= n as isize {
                continue;
            }

            let next_y = next_y as usize;
            let next_x = next_x as usize;

            if let CellStatus::Broken = grid[next_y][next_x].status {
                continue;
            }

            queue.push((0, next_y, next_x));
        }
    }

    while let Some((count, y, x)) = queue.pop() {
        //
        // 水源でも家でもないセルは多いので、
        // このループは時間がかかる
        // 打ち切る処理が必要
        //
        let end = start.elapsed();
        if end.as_millis() >= 4_000 {
            shutdown(n, &grid, &mut source);
            return;
        }

        if let CellType::Water = grid[y][x].cell_type {
            continue;
        }

        if let CellType::House = grid[y][x].cell_type {
            continue;
        }

        if let CellStatus::Broken = grid[y][x].status {
            // panic!("あり得ない");
            continue;
        }

        // eprintln!("cell {} {:?}", index, (y, x));

        let power = (c * 20).min(grid[y][x].estimated_hit_point);
        grid[y][x].estimated_hit_point -= power;

        println!("power {} c {}", power, c);

        let replay = query(y, x, power, &mut source);

        match replay {
            ExcavationResult::Complete | ExcavationResult::Illegal => return,
            ExcavationResult::Broken => {
                grid[y][x].status = CellStatus::Broken;
                grid[y][x].estimated_hit_point = 0;

                // 壊れたら隣を候補に入れる
                directions.shuffle(&mut rng);

                for &direction in &directions {
                    let next_y = y as isize + direction.0;
                    let next_x = x as isize + direction.1;

                    if next_y < 0 || next_y >= n as isize || next_x < 0 || next_x >= n as isize {
                        continue;
                    }

                    let next_y = next_y as usize;
                    let next_x = next_x as usize;

                    if let CellStatus::Broken = grid[next_y][next_x].status {
                        continue;
                    }

                    queue.push((0, next_y, next_x));
                }
            }
            ExcavationResult::NotBreak => {
                // 壊れなかったら再度queueに入れる
                queue.push((count + 1, y, x));
            }
        }
    }
}

fn shutdown<R: BufRead>(n: usize, grid: &Vec<Vec<CellData>>, source: &mut LineSource<R>) {
    // まだ掘削し終わってないセル全てを5,000で掘削し、探索を終了する

    for y in 0..n {
        for x in 0..n {
            // let mut cell = grid[y][x];

            if let CellStatus::Broken = grid[y][x].status {
                continue;
            }

            let replay = query(y, x, 5_000, source);

            match replay {
                ExcavationResult::Complete | ExcavationResult::Illegal => return,
                ExcavationResult::NotBreak => {}
                ExcavationResult::Broken => {}
            }
        }
    }
}

fn query<R: BufRead>(y: usize, x: usize, p: usize, source: &mut LineSource<R>) -> ExcavationResult {
    println!("{} {} {}", y, x, p);
    stdout().flush().unwrap();

    input! {
        from source,
        r: isize,
    }

    match r {
        0 => ExcavationResult::NotBreak,
        1 => ExcavationResult::Broken,
        2 => ExcavationResult::Complete,
        _ => ExcavationResult::Illegal,
    }
}

// fn initialize(
//     n: usize,
//     m: usize,
//     d: usize,
//     u_v_w: &mut Vec<(usize, usize, i32)>,
// ) -> Vec<Vec<usize>> {
// }

// fn to_output(plan_day: Vec<Vec<usize>>, m: usize, d: usize) -> Output {
//     let mut result = vec![0; m];

//     for day in 0..d {
//         for &load in &plan_day[day] {
//             result[load] = day + 1;
//         }
//     }

//     result
// }

// -------------------------------

// 山登り

// let mut rng = rand::thread_rng();

// let output = to_output(plan_day, m, d);

// let end = start.elapsed();

// let (score, _err, _) = compute_score(&input, &output);

// let end = start.elapsed();

// loop {
//     let end = start.elapsed();
//     if end.as_millis() >= 4_600 {
//         // eprintln!("timeup");

//         break;
//     }

//     let reply = query(x, &mut source);
// }

// println!("{}", text);
