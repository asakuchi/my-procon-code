use std::{collections::HashMap, time::Instant};

use rand::{rngs::ThreadRng, Rng};

use crate::ahc021::{ahc021_input::Ahc021Input, ahc021_output::Ahc021Output};

const BALL_COUNT: usize = 465;

pub fn after_contest(input: &Ahc021Input) -> Ahc021Output {
    let start = Instant::now();

    let mut rng = rand::thread_rng();

    let mut result = Ahc021Output {
        k: 10001,
        x_y: Vec::new(),
    };

    // 色と座標のマップをもつ
    let mut origin_map = HashMap::new();

    for i in 0..input.n {
        for j in 0..i + 1 {
            let ball = input.pyramid[i][j];

            origin_map.insert(ball, (i, j));
        }
    }

    loop {
        let end = start.elapsed();
        if end.as_millis() >= 1_900 {
            // if end.as_millis() >= 300 {
            eprintln!("time up!");

            break;
        }

        let mut tmp_result = Ahc021Output {
            k: 0,
            x_y: Vec::new(),
        };

        let mut pyramid = input.pyramid.clone();

        let mut map = origin_map.clone();

        for ball in 0..BALL_COUNT {
            loop {
                let &(i, j) = map.get(&ball).unwrap();

                let (target, t_i, t_j) = swap_upper(&mut pyramid, &mut tmp_result, i, j, &mut rng);

                // swap しなかったら target は ballと同じ
                if ball != target {
                    map.insert(ball, (t_i, t_j));
                    map.insert(target, (i, j));
                } else {
                    break;
                }
            }
        }

        if tmp_result.k < result.k {
            result = tmp_result;
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
    rnd: &mut ThreadRng,
) -> (usize, usize, usize) {
    if i == 0 {
        return (pyramid[i][j], i, j);
    }

    if j < i && j > 0 && pyramid[i][j] < pyramid[i - 1][j] && pyramid[i][j] < pyramid[i - 1][j - 1]
    {
        // ランダムにswap
        if rnd.gen::<bool>() {
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
