use petgraph::unionfind::UnionFind;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::time::Instant;

const MAX_QUEUE_SIZE: usize = 10_000;

fn main() {
    let (n, t, tiles) = input();
    Ahc011 { n, t, tiles }.main();
}

fn input() -> (usize, usize, Vec<Vec<usize>>) {
    use std::io;

    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let t: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<_>> = Vec::new();

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.chars();

        let line: Vec<_> = iter
            .map(|c| c.to_digit(16).unwrap() as usize)
            // .map(|x: usize| num::FromPrimitive::from_usize(x).unwrap())
            .collect();

        a.push(line);
    }

    (n, t, a)
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// struct Tile {
//     bit: usize,
// }

// impl Tile {

// }

struct Ahc011 {
    n: usize,
    t: usize,
    tiles: Vec<Vec<usize>>,
}

impl Ahc011 {
    fn main(&self) {
        let start = Instant::now();

        // println!("{}", self.eval(&self.tiles));

        // let new_tiles = self.change(&vec!['U', 'R', 'R', 'D', 'D', 'D']);

        // self.print(&new_tiles);

        // println!("{}", self.eval(&new_tiles));

        let mut start_point = (0, 0);

        for i in 0..self.n {
            for j in 0..self.n {
                if self.tiles[i][j] == 0 {
                    start_point = (i, j);
                    break;
                }
            }
        }

        let (start_score, max_point_at_start) = self.eval(&self.tiles, 0);

        let mut initial_hands = Vec::new();

        // max_point_at_start まで移動させる
        if max_point_at_start.0 > start_point.0 {
            let diff = max_point_at_start.0 - start_point.0;
            for _ in 0..diff {
                initial_hands.push('D');
                start_point.0 += 1;
            }
        } else if start_point.0 > max_point_at_start.0 {
            let diff = start_point.0 - max_point_at_start.0;
            for _ in 0..diff {
                initial_hands.push('U');
                start_point.0 -= 1;
            }
        }

        if max_point_at_start.1 > start_point.1 {
            let diff = max_point_at_start.1 - start_point.1;
            for _ in 0..diff {
                initial_hands.push('R');
                start_point.1 += 1;
            }
        } else if start_point.1 > max_point_at_start.1 {
            let diff = start_point.1 - max_point_at_start.1;
            for _ in 0..diff {
                initial_hands.push('L');
                start_point.1 -= 1;
            }
        }

        let mut priority_queue = BinaryHeap::new();

        priority_queue.push((start_score, initial_hands, start_point));

        let mut max_score = 0;
        let mut max_hands = Vec::new();

        while let Some((score, hands, point)) = priority_queue.pop() {
            // 時間制限
            let end = start.elapsed();
            if end.as_millis() >= 2500 {
                break;
            }

            let (x, y) = point;

            // 上方向
            if x > 0 {
                let mut new_hands = hands.clone();
                new_hands.push('U');
                let new_point = (x - 1, y);

                let new_tiles = self.change(&new_hands);
                let (new_score, _) = self.eval(&new_tiles, new_hands.len());

                // if priority_queue.len() > MAX_QUEUE_SIZE {
                //     eprintln!("over");
                // }

                if priority_queue.len() < MAX_QUEUE_SIZE || new_score > score {
                    // 一定サイズを越えてしまったら、スコアが改善するものだけ入れる
                    priority_queue.push((new_score, new_hands.clone(), new_point));
                }

                if new_score > max_score {
                    max_score = new_score;
                    max_hands = new_hands.clone();
                }
            }

            // 下方向
            if x < self.n - 1 {
                let mut new_hands = hands.clone();
                new_hands.push('D');
                let new_point = (x + 1, y);

                let new_tiles = self.change(&new_hands);
                let (new_score, _) = self.eval(&new_tiles, new_hands.len());

                if priority_queue.len() < MAX_QUEUE_SIZE || new_score > score {
                    // 一定サイズを越えてしまったら、スコアが改善するものだけ入れる
                    priority_queue.push((new_score, new_hands.clone(), new_point));
                }

                if new_score > max_score {
                    max_score = new_score;
                    max_hands = new_hands.clone();
                }
            }

            // 左方向
            if y > 0 {
                let mut new_hands = hands.clone();
                new_hands.push('L');
                let new_point = (x, y - 1);

                let new_tiles = self.change(&new_hands);
                let (new_score, _) = self.eval(&new_tiles, new_hands.len());

                if priority_queue.len() < MAX_QUEUE_SIZE || new_score > score {
                    // 一定サイズを越えてしまったら、スコアが改善するものだけ入れる
                    priority_queue.push((new_score, new_hands.clone(), new_point));
                }

                if new_score > max_score {
                    max_score = new_score;
                    max_hands = new_hands.clone();
                }
            }

            // 右方向
            if y < self.n - 1 {
                let mut new_hands = hands.clone();
                new_hands.push('R');
                let new_point = (x, y + 1);

                let new_tiles = self.change(&new_hands);
                let (new_score, _) = self.eval(&new_tiles, new_hands.len());

                if priority_queue.len() < MAX_QUEUE_SIZE || new_score > score {
                    // 一定サイズを越えてしまったら、スコアが改善するものだけ入れる
                    priority_queue.push((new_score, new_hands.clone(), new_point));
                }

                if new_score > max_score {
                    max_score = new_score;
                    max_hands = new_hands.clone();
                }
            }
        }

        self.output(&max_hands);
    }

    ///
    /// 移動
    ///
    fn change(&self, hands: &Vec<char>) -> Vec<Vec<usize>> {
        let mut point = (0, 0);

        for i in 0..self.n {
            for j in 0..self.n {
                if self.tiles[i][j] == 0 {
                    point = (i, j);
                    break;
                }
            }
        }

        let mut tiles = self.tiles.clone();

        for &hand in hands {
            let tmp = tiles[point.0][point.1];

            let change_point = match hand {
                'L' => (point.0, point.1 - 1),
                'U' => (point.0 - 1, point.1),
                'R' => (point.0, point.1 + 1),
                _ => (point.0 + 1, point.1),
            };

            tiles[point.0][point.1] = tiles[change_point.0][change_point.1];
            tiles[change_point.0][change_point.1] = tmp;

            point = change_point;
        }

        tiles
    }

    ///
    /// 評価関数
    ///
    fn eval(&self, tiles: &Vec<Vec<usize>>, hands_size: usize) -> (usize, (usize, usize)) {
        let mut max_size = 0;
        let mut max_point = (0, 0);

        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; self.n]; self.n];

        for i in 0..self.n {
            for j in 0..self.n {
                if visited[i][j] {
                    continue;
                }

                // (探索頂点, 前の頂点)
                queue.push_back(((i, j), (100, 100)));

                let mut checking_size = 0;
                let root = self.n * self.n + 1;
                let mut set = UnionFind::new(root + 1);

                // println!("search:{:?}", (i, j));

                while let Some(((x, y), (prev_x, prev_y))) = queue.pop_front() {
                    // println!(
                    //     "equiv:{} {} {}=({:?})",
                    //     set.equiv(root, x * self.n + y),
                    //     root,
                    //     x * self.n + y,
                    //     (x, y)
                    // );

                    if set.equiv(root, x * self.n + y) {
                        // 閉路あり
                        checking_size = 0;
                        break;
                    }

                    set.union(root, x * self.n + y);

                    visited[x][y] = true;
                    checking_size += 1;

                    // 上方向
                    if x > 0
                        && tiles[x][y] & 1 << 1 > 0
                        && tiles[x - 1][y] & 1 << 3 > 0
                        && (x - 1, y) != (prev_x, prev_y)
                    {
                        queue.push_back(((x - 1, y), (x, y)));
                    }

                    // 下方向
                    if x < self.n - 1
                        && tiles[x][y] & 1 << 3 > 0
                        && tiles[x + 1][y] & 1 << 1 > 0
                        && (x + 1, y) != (prev_x, prev_y)
                    {
                        queue.push_back(((x + 1, y), (x, y)));
                    }

                    // 左方向
                    if y > 0
                        && tiles[x][y] & 1 << 0 > 0
                        && tiles[x][y - 1] & 1 << 2 > 0
                        && (x, y - 1) != (prev_x, prev_y)
                    {
                        queue.push_back(((x, y - 1), (x, y)));
                    }

                    // 右方向
                    if y < self.n - 1
                        && tiles[x][y] & 1 << 2 > 0
                        && tiles[x][y + 1] & 1 << 0 > 0
                        && (x, y + 1) != (prev_x, prev_y)
                    {
                        queue.push_back(((x, y + 1), (x, y)));
                    }
                }

                if checking_size > max_size {
                    max_size = checking_size;
                    max_point = (i, j);
                }

                // println!("{:?} {}", (i, j), score);
            }
        }

        let score = if max_size == self.n * self.n - 1 {
            (500000. * (2. - hands_size as f64 / self.t as f64)).round() as usize
        } else {
            (500000. * max_size as f64 / (self.n * self.n - 1) as f64).round() as usize
        };

        (score, max_point)
    }

    // ///
    // ///  デバッグプリント
    // ///
    // fn debug_print(&self, tiles: &Vec<Vec<usize>>) {
    //     for i in 0..self.n {
    //         for j in 0..self.n {
    //             print!("{:x} ", tiles[i][j]);
    //         }
    //         println!();
    //     }
    // }

    ///
    ///  出力
    ///
    fn output(&self, result: &Vec<char>) {
        for i in 0..result.len().min(2 * self.n * self.n * self.n) {
            print!("{}", result[i]);
        }

        println!();
    }
}
