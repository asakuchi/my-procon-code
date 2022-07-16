use num_derive::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, FromPrimitive)]
#[repr(usize)]
enum Tile {
    LeftUp = 0,
    LeftDown = 1,
    RightDown = 2,
    RightUp = 3,
    LeftUpAndRightDown = 4,
    LeftDownAndRightUp = 5,
    Horizontal = 6,
    Vertical = 7,
}

impl Tile {
    fn rotate(&self) -> Self {
        match self {
            Tile::LeftUp => Tile::LeftDown,
            Tile::LeftDown => Tile::RightDown,
            Tile::RightDown => Tile::RightUp,
            Tile::RightUp => Tile::LeftUp,
            Tile::LeftUpAndRightDown => Tile::LeftDownAndRightUp,
            Tile::LeftDownAndRightUp => Tile::LeftUpAndRightDown,
            Tile::Horizontal => Tile::Vertical,
            Tile::Vertical => Tile::Horizontal,
        }
    }
}

fn main() {
    let mut tiles = input();

    let mut original_tiles = tiles.clone();

    let to: Vec<Vec<isize>> = vec![
        vec![1, 0, -1, -1],
        vec![3, -1, -1, 0],
        vec![-1, -1, 3, 2],
        vec![-1, 2, 1, -1],
        vec![1, 0, 3, 2],
        vec![3, 2, 1, 0],
        vec![2, -1, 0, -1],
        vec![-1, 3, -1, 1],
    ];

    let di: Vec<isize> = vec![0, -1, 0, 1];
    let dj: Vec<isize> = vec![-1, 0, 1, 0];

    let mut used = vec![vec![false; 30]; 30];

    let mut i_start = 0 as isize;
    let mut j_start = 0 as isize;
    let mut d = 0;

    // eprintln!("{:?}", tiles[10][8]);
    // eprintln!("{:?}", tiles[10][9]);
    // eprintln!("{:?}", tiles[11][8]);
    // eprintln!("{:?}", tiles[11][9]);
    // return;

    'search: for i in 0..15 {
        for j in 0..15 {
            match tiles[i][j] {
                Tile::Horizontal => continue,
                Tile::Vertical => continue,
                Tile::LeftUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::LeftDown => {
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::RightDown => {
                    // do nothing.
                }
                Tile::RightUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::LeftUpAndRightDown => {
                    // do nothing.
                }
                Tile::LeftDownAndRightUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
            }

            i_start = i as isize;
            j_start = j as isize;
            break 'search;
        }
    }

    used[i_start as usize][j_start as usize] = true;

    dfs(
        &mut tiles, i_start, j_start, d, 0, 4, i_start, j_start, &mut used, &to, &di, &dj, true,
        -1, -1,
    );

    // eprintln!("-------------------");

    let mut i_start = 0 as isize;
    let mut j_start = 15 as isize;
    let mut d = 0;

    'search_2: for i in 0..15 {
        for j in 15..30 {
            match tiles[i][j] {
                Tile::Horizontal => continue,
                Tile::Vertical => continue,
                Tile::LeftUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::LeftDown => {
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::RightDown => {
                    // do nothing.
                }
                Tile::RightUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
                Tile::LeftUpAndRightDown => {
                    // do nothing.
                }
                Tile::LeftDownAndRightUp => {
                    tiles[i][j] = tiles[i][j].rotate();
                    d = 2;
                }
            }

            i_start = i as isize;
            j_start = j as isize;
            break 'search_2;
        }
    }

    used[i_start as usize][j_start as usize] = true;

    dfs(
        &mut tiles, i_start, j_start, d, 15, 18, i_start, j_start, &mut used, &to, &di, &dj, true,
        -1, -1,
    );

    let mut result = vec![vec![0; 30]; 30];

    for i in 0..30 {
        for j in 0..30 {
            while original_tiles[i][j] != tiles[i][j] {
                original_tiles[i][j] = original_tiles[i][j].rotate();
                result[i][j] += 1;
            }
        }
    }

    output(&result);
}

fn input() -> Vec<Vec<Tile>> {
    use std::io;

    let stdin = io::stdin();

    let mut a: Vec<Vec<_>> = Vec::new();

    let n = 30;

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.chars();

        let line: Vec<Tile> = iter
            .map(|c| c.to_string().parse().unwrap())
            .map(|x: usize| num::FromPrimitive::from_usize(x).unwrap())
            .collect();

        a.push(line);
    }

    a
}

fn output(result: &Vec<Vec<usize>>) {
    for i in 0..30 {
        for j in 0..30 {
            print!("{}", result[i][j]);
        }
    }

    println!();
}

fn dfs(
    tiles: &mut Vec<Vec<Tile>>,
    i: isize,
    j: isize,
    d: usize,
    min_j: isize,
    max_j: isize,
    goal_i: isize,
    goal_j: isize,
    used: &mut Vec<Vec<bool>>,
    to: &Vec<Vec<isize>>,
    di: &Vec<isize>,
    dj: &Vec<isize>,
    is_start: bool,
    from_i: isize,
    from_j: isize,
) -> isize {
    // eprintln!(
    //     "{} {} goal:{} goal:{} {} {} {:?}",
    //     i, j, goal_i, goal_j, from_i, from_j, tiles[i as usize][j as usize]
    // );

    if !is_start && i == goal_i && j == goal_j {
        eprintln!("goal");

        return 100;
    }

    used[i as usize][j as usize] = true;

    let original_tile = tiles[i as usize][j as usize];
    let mut max_tile = tiles[i as usize][j as usize];

    let mut result = 0;

    let next_num = match tiles[i as usize][j as usize] {
        Tile::LeftUp => 3,
        Tile::LeftDown => 3,
        Tile::RightDown => 3,
        Tile::RightUp => 3,
        Tile::LeftUpAndRightDown => 1,
        Tile::LeftDownAndRightUp => 1,
        Tile::Horizontal => 1,
        Tile::Vertical => 1,
    };

    for next in 0..=next_num {
        for _ in 0..next {
            tiles[i as usize][j as usize] = tiles[i as usize][j as usize].rotate();
        }

        let d2: isize = to[tiles[i as usize][j as usize] as usize][d];
        if d2 == -1 {
            continue;
        }

        let next_d = ((d2 + 2) % 4) as usize;
        let next_i = i + di[d2 as usize];
        let next_j = j + dj[d2 as usize];

        if next_i < 0
            || next_i >= 30
            || next_j < 0
            || next_j >= 30
            || next_j < min_j
            || next_j >= max_j
        {
            continue;
        }

        if used[next_i as usize][next_j as usize] {
            // eprintln!("visited: {} {} from: {} {}", next_i, next_j, i, j);

            continue;
        }

        if next_i == from_i && next_j == from_j {
            continue;
        }

        used[next_i as usize][next_j as usize] = true;

        // eprintln!(
        //     "next {} {} from {} {} {} {:?} ",
        //     next_i, next_j, i, j, d, tiles[i as usize][j as usize]
        // );

        let score = dfs(
            tiles, next_i, next_j, next_d, min_j, max_j, goal_i, goal_j, used, to, di, dj, false,
            i, j,
        ) + 1;

        if score == 100 {
            return 100;
        }

        if result < score {
            result = score;
            max_tile = tiles[i as usize][j as usize];
        }

        tiles[i as usize][j as usize] = original_tile;
        used[next_i as usize][next_j as usize] = false;
    }

    // eprintln!(
    //     "end {} {} goal:{} goal:{} {} {} {:?}",
    //     i, j, goal_i, goal_j, from_i, from_j, tiles[i as usize][j as usize]
    // );

    tiles[i as usize][j as usize] = max_tile;
    result
}
