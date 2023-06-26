use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use superslice::Ext;

use crate::ahc021::{ahc021_input::Ahc021Input, ahc021_output::Ahc021Output};

// const BALL_COUNT: usize = 465;

pub fn priority_multi_block_solution(input: &Ahc021Input) -> Ahc021Output {
    let start = Instant::now();

    let mut result = Ahc021Output {
        k: 0,
        x_y: Vec::new(),
    };

    let mut pyramid = input.pyramid.clone();

    // n = 30 / block_size ごとのブロックに分けて考えてみる
    let block_size = 10;

    let mut blocks_ball = Vec::new();

    blocks_ball.push(0);

    for i in 0..input.n / block_size {
        let tmp_n = block_size * (i + 1);

        blocks_ball.push(tmp_n * (tmp_n + 1) / 2);
    }

    // eprintln!("blocks_ball {:?}", blocks_ball);

    // 色と座標のマップをもつ
    let mut map = HashMap::new();

    // let mut block_ok = vec![false; BALL_COUNT];

    let mut queue = VecDeque::new();
    // let mut queue = BinaryHeap::new();

    for i in 0..input.n {
        for j in 0..i + 1 {
            let ball = pyramid[i][j];

            map.insert(ball, (i, j));

            let block = i / block_size;

            if blocks_ball[block] <= ball && ball < blocks_ball[block + 1] {
                // OK
            } else if ball < blocks_ball[block] {
                // 下にいすぎ
                let _index = blocks_ball.lower_bound(&ball);

                queue.push_back(ball);
                // queue.push(((block as isize - index as isize).abs(), ball));
            } else if ball >= blocks_ball[block + 1] {
                // 上にいすぎ
                let _index = blocks_ball.lower_bound(&ball);

                queue.push_back(ball);
                // queue.push(((block as isize - index as isize).abs(), ball));
            }
        }
    }

    // eprintln!("map {:?}", map);

    while result.k < 10000 {
        let end = start.elapsed();
        if end.as_millis() >= 1_900 {
            // if end.as_millis() >= 300 {
            eprintln!("time up!");

            return result;
        }

        // if queue.len() != 0 && end.as_millis() < 1_000 {
        if queue.len() != 0 {
            // 前半はブロックごとに並べ替え

            // if let Some((_, ball)) = queue.pop() {
            if let Some(ball) = queue.pop_front() {
                let &(i, j) = map.get(&ball).unwrap();

                let block = i / block_size;

                if blocks_ball[block] <= ball && ball < blocks_ball[block + 1] {
                    // OK
                } else if ball < blocks_ball[block] {
                    // 下にいすぎ

                    let (target, t_i, t_j) = swap_upper(&mut pyramid, &mut result, i, j);

                    // swap しなかったら target は ballと同じ
                    if ball != target {
                        map.insert(ball, (t_i, t_j));
                        map.insert(target, (i, j));

                        let new_block = t_i / block_size;

                        let index = blocks_ball.lower_bound(&ball);

                        // 不十分なら再度queueに
                        if (new_block as isize - index as isize).abs() > 1 {
                            queue.push_back(ball);
                            // queue.push(((new_block as isize - index as isize).abs(), ball));
                        }

                        // // 不十分なら再度queueに
                        // if !(blocks_ball[new_block] <= ball && ball < blocks_ball[new_block + 1]) {
                        //     // queue.push_back(ball);

                        //     let index = blocks_ball.lower_bound(&ball);
                        //     queue.push(((new_block as isize - index as isize).abs(), ball));
                        // }

                        // // swap相手についても
                        // if !(blocks_ball[block] <= target && target < blocks_ball[block + 1]) {
                        //     queue.push_back(target);
                        // }
                    }
                } else if ball >= blocks_ball[block + 1] {
                    // 上にいすぎ

                    let (target, t_i, t_j) = swap_lower(&mut pyramid, &mut result, i, j);

                    // swap しなかったら target は ballと同じ
                    if ball != target {
                        map.insert(ball, (t_i, t_j));
                        map.insert(target, (i, j));

                        let new_block = t_i / block_size;

                        let index = blocks_ball.lower_bound(&ball);

                        // 不十分なら再度queueに
                        if (new_block as isize - index as isize).abs() > 1 {
                            // queue.push(((new_block as isize - index as isize).abs(), ball));
                            queue.push_back(ball);
                        }

                        // // 不十分なら再度queueに
                        // if !(blocks_ball[new_block] <= ball && ball < blocks_ball[new_block + 1]) {
                        //     // queue.push_back(ball);

                        //     let index = blocks_ball.lower_bound(&ball);
                        //     queue.push(((new_block as isize - index as isize).abs(), ball));
                        // }

                        // // swap相手についても
                        // if !(blocks_ball[block] <= target && target < blocks_ball[block + 1]) {
                        //     queue.push_back(target);
                        // }
                    }
                }
            }

            if queue.len() == 0 {
                eprintln!("empty time {:?} k {}", start.elapsed(), result.k);
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
