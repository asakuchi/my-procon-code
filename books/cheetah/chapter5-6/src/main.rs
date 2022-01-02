// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        east: usize,
        west: usize,
        south: usize,
        north: usize,
    }

    let board_size = 100; // nに対して十分大きい数
    let mut memo = vec![vec![false; board_size]; board_size];

    let east = east as f64 / 100.0;
    let west = west as f64 / 100.0;
    let south = south as f64 / 100.0;
    let north = north as f64 / 100.0;

    let initial_x = 50;
    let initial_y = 50;
    memo[initial_x][initial_y] = true;
    let probability = dfs(n, initial_x, initial_y, east, west, south, north, &mut memo);

    println!("{}", probability);
}

fn dfs(
    depth: usize,
    x: usize,
    y: usize,
    east: f64,
    west: f64,
    south: f64,
    north: f64,
    memo: &mut std::vec::Vec<std::vec::Vec<bool>>,
) -> f64 {
    if depth == 0 {
        return 1.0;
    }

    let mut probability = 0.0;

    // east
    if !memo[x + 1][y] {
        memo[x + 1][y] = true;

        let p = dfs(depth - 1, x + 1, y, east, west, south, north, memo) * east;
        probability += p;

        memo[x + 1][y] = false;
    }

    // west
    if !memo[x - 1][y] {
        memo[x - 1][y] = true;

        let p = dfs(depth - 1, x - 1, y, east, west, south, north, memo) * west;
        probability += p;

        memo[x - 1][y] = false;
    }

    // south
    if !memo[x][y - 1] {
        memo[x][y - 1] = true;

        let p = dfs(depth - 1, x, y - 1, east, west, south, north, memo) * south;
        probability += p;

        memo[x][y - 1] = false;
    }

    // north
    if !memo[x][y + 1] {
        memo[x][y + 1] = true;

        let p = dfs(depth - 1, x, y + 1, east, west, south, north, memo) * north;
        probability += p;

        memo[x][y + 1] = false;
    }

    probability
}
