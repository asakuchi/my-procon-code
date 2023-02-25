// use num_derive::FromPrimitive;
use proconio::{input, marker::Chars};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Computer(usize),
    Cable,
    None,
}

#[derive(Clone, Debug)]
pub struct Input {
    n: usize,
    k: usize,
    c: Vec<Vec<Tile>>,
}

fn main() {
    let input = input();

    let mut x = Vec::new();
    let mut y = Vec::new();

    let mut visited = vec![vec![false; input.n]; input.n];

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    // 貪欲的に隣接しているものを探す
    for i in 0..input.n {
        for j in 0..input.n {
            if visited[i][j] {
                // continue;
            }

            if input.c[i][j] == Tile::None {
                continue;
            }

            for &(d_i, d_j) in &directions {
                let next_i = i as isize + d_i;
                let next_j = j as isize + d_j;

                if next_i < 0
                    || next_j < 0
                    || next_i >= input.n as isize
                    || next_j >= input.n as isize
                {
                    continue;
                }

                let (next_i, next_j) = (next_i as usize, next_j as usize);

                if visited[next_i][next_j] {
                    continue;
                }

                if input.c[next_i][next_j] == Tile::None {
                    continue;
                }

                if input.c[i][j] == input.c[next_i][next_j] {
                    visited[i][j] = true;
                    visited[next_i][next_j] = true;

                    // result
                    y.push((i, j, next_i, next_j));
                }
            }
        }
    }

    output(&x, &y);
}

fn input() -> Input {
    input! {
        n: usize,
        k: usize,
        c_org: [Chars; n],
    }

    let mut c = vec![vec![Tile::None; n]; n];

    for i in 0..n {
        for j in 0..n {
            c[i][j] = match c_org[i][j].to_digit(10).unwrap() as usize {
                0 => Tile::None,
                value => Tile::Computer(value),
            };
        }
    }

    Input { n, k, c }
}

fn output(x: &Vec<(usize, usize, usize, usize)>, y: &Vec<(usize, usize, usize, usize)>) {
    println!("{}", x.len());

    for &(a, b, c, d) in x {
        println!("{} {} {} {}", a, b, c, d);
    }

    println!("{}", y.len());

    for &(a, b, c, d) in y {
        println!("{} {} {} {}", a, b, c, d);
    }
}
