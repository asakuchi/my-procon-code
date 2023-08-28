use itertools::Itertools;
use proconio::{input, source::line::LineSource};
use std::{
    collections::HashMap,
    io::{stdin, stdout, BufReader, Write},
};

#[derive(Clone, Copy, Debug)]
struct CellData {
    number: usize,
    center: f64,
    // 以下は正負がわかる程度の想定
    right: f64,
    left: f64,
    up: f64,
    down: f64,
}

impl CellData {
    fn new(number: usize) -> CellData {
        CellData {
            number,
            center: 0.,
            right: 0.,
            left: 0.,
            up: 0.,
            down: 0.,
        }
    }
}

// enum Direction {
//     LeftUp,
//     RightUp,
//     RightDown,
//     LeftDown,
// }

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));

    input! {
        from &mut source,
        l: usize,
        n: usize,
        _s: usize,
        y_x: [(usize, usize); n],
    }

    let directions = vec![(0, 0), (-3, 0), (0, 3), (3, 0), (0, -3)];

    //
    // 配置
    //

    let p = place(l);

    //
    //
    //

    let mut exit_cell_map = HashMap::new();

    for (i, &(y, x)) in y_x.iter().enumerate() {
        exit_cell_map.insert((y, x), CellData::new(i));
    }

    for (&(i, j), value) in exit_cell_map.iter_mut() {
        value.center = p[i][j] as f64;

        for direct_index in 1..directions.len() {
            let direction = directions[direct_index];

            let next_i = (i as isize + l as isize + direction.0) % l as isize;
            let next_j = (j as isize + l as isize + direction.1) % l as isize;

            // let p_n = value.center - p[next_i as usize][next_j as usize] as f64 > 0.;

            println!(
                "# direct {} ({},{}) {} next ({},{}) {} ",
                direct_index,
                i,
                j,
                value.center,
                next_i,
                next_j,
                p[next_i as usize][next_j as usize],
            );

            match direct_index {
                0 => value.center = p[next_i as usize][next_j as usize] as f64,
                1 => value.up = p[next_i as usize][next_j as usize] as f64,
                2 => value.right = p[next_i as usize][next_j as usize] as f64,
                3 => value.down = p[next_i as usize][next_j as usize] as f64,
                _ => value.left = p[next_i as usize][next_j as usize] as f64,
            }
        }
    }

    stdout().flush().unwrap();

    //
    // 計測
    //

    // let mut predicts: Vec<_> = (0..n).map(|i| CellData::new(i)).collect();
    // let mut predict = vec![CellData::new(); n];
    let mut predicts = Vec::new();

    for i in 0..n {
        let mut predict = CellData::new(i);

        {
            let mut data = 0.;

            let call_size = 60;
            let direct_index = 0;

            for _ in 0..call_size {
                let direction = directions[direct_index];
                println!("{} {} {}", i, direction.0, direction.1);
                stdout().flush().unwrap();

                input! {
                    from &mut source,
                    m: f64,
                }

                data += m;

                println!("# m: {}", m);
            }

            let mean = data / call_size as f64;
            predict.center = mean;
        }

        for direct_index in 1..directions.len() {
            let mut data = 0.;

            let call_size = 10;

            for _ in 0..call_size {
                let direction = directions[direct_index];
                println!("{} {} {}", i, direction.0, direction.1);
                stdout().flush().unwrap();

                input! {
                    from &mut source,
                    m: f64,
                }

                data += m;

                println!("# m: {}", m);
            }

            let mean = data / call_size as f64;

            match direct_index {
                0 => predict.center = mean,
                1 => predict.up = mean,
                2 => predict.right = mean,
                3 => predict.down = mean,
                _ => predict.left = mean,
            }
        }

        predicts.push(predict);
        println!("# predict[{}]: {:?}", i, predicts[i]);
    }

    //
    // y と predict の 近いところを探す
    //

    // let mut predict_index: Vec<_> = predict.iter().enumerate().collect();
    let exit_cells: Vec<_> = exit_cell_map.values().collect();

    println!("# predicts");
    println!("# {:?}", predicts);
    println!("# exit_cells");
    println!("# {:?}", exit_cells);

    println!("-1 -1 -1");
    stdout().flush().unwrap();

    let mut result = vec![0; n];

    //
    // // TODO: compare
    //

    for predict in predicts.iter() {
        // 向きまで一致している候補
        let mut best_index = None;
        let mut best_diff = std::f64::MAX;

        // 向きは一致していない候補
        let mut second_index = 0;
        let mut second_diff = std::f64::MAX;

        println!("# wormhole_i {:?}", predict);

        for &exit_cell in exit_cells.iter() {
            let diff = (predict.center - exit_cell.center).abs();

            // if exit_cell.number == 79 {
            //     println!("# {:?}", exit_cell);
            //     println!(
            //         "# up {}",
            //         (predict.up - predict.center > 0.) == (exit_cell.up - exit_cell.center > 0.)
            //     );
            //     println!(
            //         "# down {}",
            //         (predict.down - predict.center > 0.)
            //             == (exit_cell.down - exit_cell.center > 0.)
            //     );
            //     println!(
            //         "# left {}",
            //         (predict.left - predict.center > 0.)
            //             == (exit_cell.left - exit_cell.center > 0.)
            //     );
            //     println!(
            //         "# right {}",
            //         (predict.right - predict.center > 0.)
            //             == (exit_cell.right - exit_cell.center > 0.)
            //     );
            // }

            // if (predict.up - predict.center > 0.) == (exit_cell.up - exit_cell.center > 0.)
            //     && (predict.down - predict.center > 0.) == (exit_cell.down - exit_cell.center > 0.)
            //     && (predict.left - predict.center > 0.) == (exit_cell.left - exit_cell.center > 0.)
            //     && (predict.right - predict.center > 0.)
            //         == (exit_cell.right - exit_cell.center > 0.)
            // {
            //     if diff < best_diff {
            //         best_diff = diff;
            //         best_index = Some(exit_cell.number);

            //         // println!(
            //         //     "# update best exit_cell.number {} diff {} ",
            //         //     exit_cell.number, best_diff
            //         // );
            //     }
            // } else {
            // 高さだけで評価
            if diff < second_diff {
                second_diff = diff;
                second_index = exit_cell.number;

                // println!(
                //     "# update second exit_cell.number {} diff {} ",
                //     exit_cell.number, second_diff
                // );
            }
            // }
        }

        //  i 番目の ワームホール に入ると、 Ai​  番目の 出口セル に出る
        let wormhole_i = predict.number;

        if let Some(best_exit_cell) = best_index {
            println!(
                "# best wormhole {}  exit_cell {} ",
                wormhole_i, best_exit_cell
            );

            result[wormhole_i] = best_exit_cell;
        } else {
            println!(
                "# second wormhole {} exit_cell {} ",
                wormhole_i, second_index
            );

            result[wormhole_i] = second_index;
        }
    }

    // for i in 0..n {
    //     let wormhole_i = predict_index[i].0;
    //     let exit_cell_i = exit_cells[i].0;

    //     result[wormhole_i] = exit_cell_i;
    // }

    for i in 0..n {
        println!("{}", result[i]);
    }

    stdout().flush().unwrap();
}

// fn place(l: usize) -> Vec<Vec<usize>> {
//     let mut p = Vec::new();

//     let max = 1_000;

//     let mut create_line = |j: usize| {
//         let mut line = Vec::new();

//         for i in 0..l / 2 {
//             // line.push(j + i);
//             line.push(j * max / l + i * max / l);
//         }

//         if l % 2 == 1 {
//             // line.push(j + l / 2);
//             line.push(j + max);
//         }

//         for i in (0..l / 2).rev() {
//             // line.push(j + i);
//             line.push(j * max / l + i * max / l);
//         }

//         println!("{}", line.iter().format(" "));

//         p.push(line);
//     };

//     for j in 0..l / 2 {
//         create_line(j);
//     }

//     if l % 2 == 1 {
//         let j = l / 2;
//         create_line(j);
//     }

//     for j in (0..l / 2).rev() {
//         create_line(j);
//     }

//     // ---------------------

//     // for i in 0..l {
//     //     let mut line = Vec::new();

//     //     for j in 0..l {
//     //         line.push((i * l + j) % 1_000);
//     //     }

//     //     println!("{}", line.iter().format(" "));
//     //     p.push(line);
//     // }

//     p
// }

fn place(l: usize) -> Vec<Vec<usize>> {
    let mut p = Vec::new();

    let mut create_line = |j: usize, diff: usize| {
        let mut line = Vec::new();

        let max = 500;
        let line_plus = 13;
        let reverse_plus = 17;

        for i in 0..l / 2 {
            // line.push(j + i);
            line.push(j * (max / l + line_plus) + i * max / l + diff);
        }

        if l % 2 == 1 {
            // line.push(j + l / 2);
            line.push(j + max + line_plus + diff);
        }

        for i in (0..l / 2).rev() {
            // line.push(j + i);
            line.push(j * (max / l + line_plus) + i * max / l + reverse_plus + diff);
        }

        println!("{}", line.iter().format(" "));

        p.push(line);
    };

    for j in 0..l / 2 {
        create_line(j, 0);
    }

    if l % 2 == 1 {
        let j = l / 2;
        create_line(j, 31);
    }

    for j in (0..l / 2).rev() {
        create_line(j, 89);
    }

    // ---------------------

    // for i in 0..l {
    //     let mut line = Vec::new();

    //     for j in 0..l {
    //         line.push((i * l + j) % 1_000);
    //     }

    //     println!("{}", line.iter().format(" "));
    //     p.push(line);
    // }

    p
}
