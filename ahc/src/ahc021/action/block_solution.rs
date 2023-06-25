use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use crate::ahc021::{ahc021_input::Ahc021Input, ahc021_output::Ahc021Output};

// const BALL_COUNT: usize = 465;

pub fn block_solution(input: &Ahc021Input) -> Ahc021Output {
    let start = Instant::now();

    let mut result = Ahc021Output {
        k: 0,
        x_y: Vec::new(),
    };

    let mut pyramid = input.pyramid.clone();

    // 色と座標のマップをもつ
    let mut map = HashMap::new();

    // let mut block_ok = vec![false; BALL_COUNT];

    let mut queue = VecDeque::new();

    for i in 0..input.n {
        for j in 0..i + 1 {
            let ball = pyramid[i][j];

            map.insert(ball, (i, j));

            if ball < 55 {
                // 上部にいるべき
                if !(i < 10) {
                    // block_ok[ball] = true;
                    queue.push_back(ball);
                }
            } else if ball < 210 {
                // 中部にいるべき
                if !(10 <= i && i < 20) {
                    // block_ok[ball] = true;
                    queue.push_back(ball);
                }
            } else {
                // 下部にいるべき
                if !(20 <= i) {
                    // block_ok[ball] = true;
                    queue.push_back(ball);
                }
            }
        }
    }

    // eprintln!("map {:?}", map);

    // n = 30 なので、以下の3つのブロックに分けて考えてみる
    // 0 <= x < 10  （0〜54）
    // 10 <= x < 20 （55〜209）
    // 20 <= x < 30 （210〜464）

    // 愚直解
    while result.k < 10000 {
        let end = start.elapsed();
        if end.as_millis() >= 1_900 {
            eprintln!("time up!");

            return result;
        }

        if queue.len() != 0 && end.as_millis() < 1_000 {
            // 前半はブロックごとに並べ替え

            if let Some(ball) = queue.pop_front() {
                let &(i, j) = map.get(&ball).unwrap();

                if ball < 55 {
                    // 上部にいるべき
                    let (target, t_i, t_j) = swap_upper(&mut pyramid, &mut result, i, j);

                    // swap しなかったら target は ballと同じ
                    if ball != target {
                        map.insert(ball, (t_i, t_j));
                        map.insert(target, (i, j));

                        // 不十分なら再度queueに
                        if !(t_i < 10) {
                            queue.push_back(ball);
                        }
                    }
                } else if ball < 210 {
                    // 中部にいるべき

                    if i < 10 {
                        let (target, t_i, t_j) = swap_lower(&mut pyramid, &mut result, i, j);

                        // swap しなかったら target は ballと同じ
                        if ball != target {
                            map.insert(ball, (t_i, t_j));
                            map.insert(target, (i, j));

                            // 不十分なら再度queueに
                            if !(10 <= t_i && t_i < 20) {
                                queue.push_back(ball);
                            }
                        }
                    } else if 20 <= i {
                        let (target, t_i, t_j) = swap_upper(&mut pyramid, &mut result, i, j);

                        // swap しなかったら target は ballと同じ
                        if ball != target {
                            map.insert(ball, (t_i, t_j));
                            map.insert(target, (i, j));

                            // 不十分なら再度queueに
                            if !(10 <= t_i && t_i < 20) {
                                queue.push_back(ball);
                            }
                        }
                    }
                } else {
                    // 下部にいるべき
                    let (target, t_i, t_j) = swap_lower(&mut pyramid, &mut result, i, j);

                    // swap しなかったら target は ballと同じ
                    if ball != target {
                        map.insert(ball, (t_i, t_j));
                        map.insert(target, (i, j));

                        // 不十分なら再度queueに
                        if !(t_i < 10) {
                            queue.push_back(ball);
                        }
                    }

                    // 不十分なら再度queueに
                    if !(20 <= t_i) {
                        // block_ok[ball] = true;
                        queue.push_back(ball);
                    }
                }
            }

            if queue.len() == 0 {
                eprintln!("empty time {:?}", start.elapsed());
            }
        } else {
            // 後半はバブルソート
            for i in 1..input.n {
                for j in 0..i + 1 {
                    // println!("{}", pyramid[i][j]);

                    swap_upper(&mut pyramid, &mut result, i, j);
                }
            }
        }
    }

    eprintln!("k {:?}", result.k);
    eprintln!("time {:?}", start.elapsed());

    result
}

fn swap_upper(
    pyramid: &mut Vec<Vec<usize>>,
    result: &mut Ahc021Output,
    i: usize,
    j: usize,
) -> (usize, usize, usize) {
    if i == 0 {
        return (pyramid[i][j], i, j);
    }

    // より大きい方とswapしたい
    if j < i && j > 0 && pyramid[i][j] < pyramid[i - 1][j] && pyramid[i][j] < pyramid[i - 1][j - 1]
    {
        if pyramid[i - 1][j] > pyramid[i - 1][j - 1] {
            // 右上方向と swap
            let target = pyramid[i - 1][j];

            pyramid[i - 1][j] = pyramid[i][j];
            pyramid[i][j] = target;

            result.add(i, j, i - 1, j);

            return (target, i - 1, j);
        } else {
            // 左上方向と swap
            let target = pyramid[i - 1][j - 1];

            pyramid[i - 1][j - 1] = pyramid[i][j];
            pyramid[i][j] = target;

            result.add(i, j, i - 1, j - 1);

            return (target, i - 1, j - 1);
        }
    } else if j < i && pyramid[i][j] < pyramid[i - 1][j] {
        // 右上方向と swap
        let target = pyramid[i - 1][j];

        pyramid[i - 1][j] = pyramid[i][j];
        pyramid[i][j] = target;

        result.add(i, j, i - 1, j);

        return (target, i - 1, j);
    } else if j > 0 && pyramid[i][j] < pyramid[i - 1][j - 1] {
        // 左上方向と swap
        let target = pyramid[i - 1][j - 1];

        pyramid[i - 1][j - 1] = pyramid[i][j];
        pyramid[i][j] = target;

        result.add(i, j, i - 1, j - 1);

        return (target, i - 1, j - 1);
    }

    (pyramid[i][j], i, j)
}

fn swap_lower(
    pyramid: &mut Vec<Vec<usize>>,
    result: &mut Ahc021Output,
    i: usize,
    j: usize,
) -> (usize, usize, usize) {
    if i == 29 {
        return (pyramid[i][j], i, j);
    }

    // より小さい方とswapしたい
    if pyramid[i][j] > pyramid[i + 1][j] && pyramid[i][j] > pyramid[i + 1][j + 1] {
        if pyramid[i + 1][j] < pyramid[i + 1][j + 1] {
            // 左下方向と swap
            let target = pyramid[i + 1][j];

            pyramid[i + 1][j] = pyramid[i][j];
            pyramid[i][j] = target;

            result.add(i, j, i + 1, j);

            return (target, i + 1, j);
        } else {
            // 右下方向と swap
            let target = pyramid[i + 1][j + 1];

            pyramid[i + 1][j + 1] = pyramid[i][j];
            pyramid[i][j] = target;

            result.add(i, j, i + 1, j + 1);

            return (target, i + 1, j + 1);
        }
    } else if pyramid[i][j] > pyramid[i + 1][j] {
        // 左下方向と swap
        let target = pyramid[i + 1][j];

        pyramid[i + 1][j] = pyramid[i][j];
        pyramid[i][j] = target;

        result.add(i, j, i + 1, j);

        return (target, i + 1, j);
    } else if pyramid[i][j] > pyramid[i + 1][j + 1] {
        // 右下方向と swap
        let target = pyramid[i + 1][j + 1];

        pyramid[i + 1][j + 1] = pyramid[i][j];
        pyramid[i][j] = target;

        result.add(i, j, i + 1, j + 1);

        return (target, i + 1, j + 1);
    }

    (pyramid[i][j], i, j)
}
