use std::time::Instant;

use crate::ahc021::{ahc021_input::Ahc021Input, ahc021_output::Ahc021Output};

pub fn first_solution(input: &Ahc021Input) -> Ahc021Output {
    let start = Instant::now();

    let mut result = Ahc021Output {
        k: 0,
        x_y: Vec::new(),
    };

    let mut pyramid = input.pyramid.clone();

    // 愚直解
    while result.k < 10000 {
        let end = start.elapsed();
        if end.as_millis() >= 1_900 {
            eprintln!("time up!");

            return result;
        }

        // バブルソート
        for i in 1..input.n {
            for j in 0..i + 1 {
                // println!("{}", pyramid[i][j]);

                if j < i && pyramid[i][j] < pyramid[i - 1][j] {
                    // 右上方向と swap
                    let tmp = pyramid[i][j];
                    pyramid[i][j] = pyramid[i - 1][j];
                    pyramid[i - 1][j] = tmp;

                    result.add(i, j, i - 1, j);
                } else if j > 0 && pyramid[i][j] < pyramid[i - 1][j - 1] {
                    // 左上方向と swap
                    let tmp = pyramid[i][j];
                    pyramid[i][j] = pyramid[i - 1][j - 1];
                    pyramid[i - 1][j - 1] = tmp;

                    result.add(i, j, i - 1, j - 1);
                }
            }
        }
    }

    eprintln!("k {:?}", result.k);
    eprintln!("time {:?}", start.elapsed());

    result
}
