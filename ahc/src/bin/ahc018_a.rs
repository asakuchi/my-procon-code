// for interactive
use proconio::{input, source::line::LineSource};
use std::cmp::Reverse;
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

    let mut directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = BinaryHeap::new();

    // 水源をqueueに入れる
    for i in 0..w {
        let (y, x) = water_source[i];

        directions.shuffle(&mut rng);

        queue.push((Reverse(0), y, x));
    }

    // 家をqueueに入れる
    for i in 0..k {
        let (y, x) = house[i];

        directions.shuffle(&mut rng);

        queue.push((Reverse(0), y, x));
    }

    while let Some((Reverse(count), y, x)) = queue.pop() {
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

        if let CellStatus::Broken = grid[y][x].status {
            continue;
        }

        // println!("count {} y: {} x: {}", count, y, x);

        // 水源と家は掘削完了させる
        if let CellType::Water = grid[y][x].cell_type {
            if beat_down(c, &mut grid, y, x, &mut source) {
                return;
            }
        } else if let CellType::House = grid[y][x].cell_type {
            if beat_down(c, &mut grid, y, x, &mut source) {
                return;
            }
        } else {
            let power = (c * 20).min(grid[y][x].estimated_hit_point);
            grid[y][x].estimated_hit_point -= power;

            // println!("power {} c {}", power, c);

            let replay = query(y, x, power, &mut source);

            match replay {
                ExcavationResult::Complete | ExcavationResult::Illegal => return,
                ExcavationResult::Broken => {
                    // 次の文へ
                }
                ExcavationResult::NotBreak => {
                    // 壊れなかったら再度queueに入れる
                    queue.push((Reverse(count + 1), y, x));
                    continue;
                }
            }
        }

        // 掘削完了
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

            queue.push((Reverse(0), next_y, next_x));
        }
    }
}

fn shutdown<R: BufRead>(n: usize, grid: &Vec<Vec<CellData>>, source: &mut LineSource<R>) {
    // まだ掘削し終わってないセル全てを5,000で掘削し、探索を終了する

    for y in 0..n {
        for x in 0..n {
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

///
/// 掘削完了まで続ける
///
/// true を返したらプログラム終了
///
fn beat_down<R: BufRead>(
    c: usize,
    grid: &mut Vec<Vec<CellData>>,
    y: usize,
    x: usize,
    source: &mut LineSource<R>,
) -> bool {
    loop {
        // 壊れるまでやる

        let power = (c * 3).min(grid[y][x].estimated_hit_point);
        grid[y][x].estimated_hit_point -= power;

        let replay = query(y, x, power, source);

        match replay {
            ExcavationResult::Complete | ExcavationResult::Illegal => return true,
            ExcavationResult::Broken => {
                grid[y][x].status = CellStatus::Broken;
                grid[y][x].estimated_hit_point = 0;
                break;
            }
            ExcavationResult::NotBreak => {}
        }
    }

    false
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
