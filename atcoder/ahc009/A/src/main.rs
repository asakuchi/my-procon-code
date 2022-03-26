// use proconio::derive_readable;
// use proconio::marker::Chars;
// use proconio::marker::Usize1;
// use itertools::izip;
// use itertools::Itertools;
// use petgraph::unionfind::UnionFind;
use rand::prelude::*;

const UP: &str = "U";
const DOWN: &str = "D";
const LEFT: &str = "L";
const RIGHT: &str = "R";

const MAP_SIZE: usize = 20;

#[derive(Copy, Clone, Debug)]
enum Cell {
    /// 未踏
    Unexplored,
    Step(usize, (usize, usize)),
    /// 通行不可
    Unavailable,
}

fn main() {
    let (si, sj, ti, tj, p, h, v) = input();

    let mut rng = rand::thread_rng();

    let mut result = Vec::new();

    let mut random = Vec::new();

    for _ in 0..200 {
        random.push(UP);
        random.push(LEFT);
        random.push(DOWN);
        random.push(RIGHT);
    }

    random.shuffle(&mut rng);

    let shortest_path = search_shortest_path(si, sj, ti, tj, p, &h, &v);

    for i in 1..shortest_path.len() {
        let diff = (
            shortest_path[i].0 as isize - shortest_path[i - 1].0 as isize,
            shortest_path[i].1 as isize - shortest_path[i - 1].1 as isize,
        );

        let direction = match diff {
            (1, 0) => DOWN,
            (-1, 0) => UP,
            (0, 1) => RIGHT,
            (_, _) => LEFT,
        };

        result.push(direction);

        if p > 2.0 {
            let y: f64 = rng.gen();
            if y < p / 2. {
                result.push(direction);
            }
        }
    }

    // ------- random -------

    let mut random = Vec::new();

    for _ in 0..500 {
        random.push(DOWN);
        random.push(RIGHT);
    }

    for _ in 0..200 {
        random.push(LEFT);
    }

    for _ in 0..100 {
        random.push(UP);
    }

    random.shuffle(&mut rng);

    result.append(&mut random);

    // ------- random -------

    output(&result);
}

fn input() -> (
    usize,
    usize,
    usize,
    usize,
    f64,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
) {
    use std::io;

    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let si: usize = iter.next().unwrap().parse().unwrap();
    let sj: usize = iter.next().unwrap().parse().unwrap();
    let ti: usize = iter.next().unwrap().parse().unwrap();
    let tj: usize = iter.next().unwrap().parse().unwrap();
    let p: f64 = iter.next().unwrap().parse().unwrap();

    let mut h: Vec<Vec<usize>> = Vec::new();

    for _ in 0..20 {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let nums: Vec<usize> = buf
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();

        h.push(nums);
    }

    let mut v: Vec<Vec<usize>> = Vec::new();

    for _ in 0..19 {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let nums: Vec<usize> = buf
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();

        v.push(nums);
    }

    // eprintln!("{} {} {} {} {} {:?} {:?}", si, sj, ti, tj, p, h, v);

    (si, sj, ti, tj, p, h, v)
}

fn random_result(
    si: usize,
    sj: usize,
    ti: usize,
    tj: usize,
    p: f64,
    h: &Vec<Vec<usize>>,
    v: &Vec<Vec<usize>>,
) {
}

fn search_shortest_path(
    si: usize,
    sj: usize,
    ti: usize,
    tj: usize,
    p: f64,
    h: &Vec<Vec<usize>>,
    v: &Vec<Vec<usize>>,
) -> Vec<(usize, usize)> {
    // 最短経路 ---------------------------

    let patterns = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((si, sj));

    let mut steps = vec![vec![Cell::Unexplored; 20]; 20];
    steps[si][sj] = Cell::Step(0, (0, 0));

    while let Some(target) = queue.pop_front() {
        // eprintln!("target:{:?}", target);

        for pattern in patterns.iter() {
            let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

            // eprintln!("next:{:?}", next);

            // 終了条件
            if 0 > next.0
                || next.0 >= MAP_SIZE as isize
                || 0 > next.1
                || next.1 >= MAP_SIZE as isize
            {
                continue;
            }

            // eprintln!("next:{:?} checkwall", next);

            let next = (next.0 as usize, next.1 as usize);

            if target.0 != next.0 {
                let vertical_wall = v[target.0.min(next.0)][target.1];

                if vertical_wall == 1 {
                    continue;
                }
            }

            if target.1 != next.1 {
                let horizen_wall = h[target.0][target.1.min(next.1)];

                if horizen_wall == 1 {
                    continue;
                }
            }

            // eprintln!("next:{:?} Unexplored? {:?}", next, steps[next.0][next.1]);

            // 次へ
            if let Cell::Unexplored = steps[next.0][next.1] {
                if let Cell::Step(step, _from) = steps[target.0][target.1] {
                    steps[next.0][next.1] = Cell::Step(step + 1, target);
                    queue.push_back(next);

                    // eprintln!("next:{:?} pushed", next);
                }
            }
        }
    }

    // eprintln!("search shortest_path");
    // eprintln!("steps:{:?}", steps);

    let mut shortest_path = Vec::new();
    shortest_path.push((ti, tj));

    {
        let mut current = (ti, tj);

        while current != (si, sj) {
            // eprintln!("current:{:?}", current);

            if let Cell::Step(_step, from) = steps[current.0][current.1] {
                current = from;
                shortest_path.push(from);
            } else {
                break;
            }
        }
    }

    shortest_path.reverse();

    // eprintln!("{:?}", shortest_path);

    shortest_path
}

fn output(result: &Vec<&str>) {
    // eprintln!("{:?}", result);
    // eprintln!("{:?}", &result[0..200]);

    println!(
        "{}",
        &result[0..200]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("")
    );
}
