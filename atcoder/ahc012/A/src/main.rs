use rand::Rng;
use std::collections::BinaryHeap;
// use std::collections::VecDeque;
use rand::seq::SliceRandom;
use std::time::Instant;

pub type Output = Vec<(i32, i32, i32, i32)>;

const POINT_MAX: i32 = 1_000_000_000;
const NARROW_POINT: i32 = 1_000;

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub k: usize,
    pub xy: Vec<(i32, i32)>,
    pub a: Vec<i32>,
}

fn main() {
    let start = Instant::now();

    let mut rng = rand::thread_rng();
    // let i: i32 = rng.gen();
    // println!("Integer: {}", rng.gen_range(-POINT_MAX, POINT_MAX));

    // return;

    let input = input();

    let initial_lines = initialize_lines(input.k);
    let (initial_score, _error, (_b, _pieces, not_assigned)) =
        compute_score(&input, &initial_lines);
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((initial_score, initial_lines.clone(), not_assigned));

    let mut max_score = initial_score;
    let mut result = initial_lines.clone();

    // eprintln!("start!! {}", initial_score);

    while let Some((_current_score, current_lines, current_not_assigned)) = priority_queue.pop() {
        // 時間制限
        let end = start.elapsed();
        if end.as_millis() >= 2850 {
            // eprintln!("timeup");

            break;
        }

        let mut candi = Vec::new();

        // for i in 0..input.k {

        let mut choice_num = (0..current_lines.len()).collect::<Vec<_>>();
        choice_num.shuffle(&mut rng);

        // for i in 0..current_lines.len() {
        for i in 0..3 {
            let i = choice_num[i];

            let mut lines = current_lines.clone();

            // rng.gen_range(-POINT_MAX, POINT_MAX)

            lines[i].0 += rng.gen_range(-100, 100);

            if lines[i].0 >= POINT_MAX {
                lines[i].0 = POINT_MAX;
            } else if lines[i].0 <= -POINT_MAX {
                lines[i].0 = -POINT_MAX;
            }

            lines[i].1 += rng.gen_range(-100, 100);

            if lines[i].1 >= POINT_MAX {
                lines[i].1 = POINT_MAX;
            } else if lines[i].1 <= -POINT_MAX {
                lines[i].1 = -POINT_MAX;
            }

            lines[i].2 += rng.gen_range(-100, 100);

            if lines[i].2 >= POINT_MAX {
                lines[i].2 = POINT_MAX;
            } else if lines[i].2 <= -POINT_MAX {
                lines[i].2 = -POINT_MAX;
            }

            lines[i].3 += rng.gen_range(-100, 100);

            if lines[i].3 >= POINT_MAX {
                lines[i].3 = POINT_MAX;
            } else if lines[i].3 <= -POINT_MAX {
                lines[i].3 = -POINT_MAX;
            }

            let (score, _error, (_b, _pieces, not_assigned)) = compute_score(&input, &lines);

            candi.push((score, lines.clone(), not_assigned.clone()));

            // println!("score:{}", score);

            if score > max_score {
                max_score = score;
                result = lines.clone();

                // println!("max更新:{}", score);
            }
        }

        // for _ in 0..2 {
        if input.k > current_lines.len() {
            let mut searching = current_not_assigned.clone();

            searching.sort_by_key(|x| std::cmp::Reverse(x.len()));

            let mut iter = searching.iter();

            for _ in 0..8 {
                let mut lines = current_lines.clone();

                if let Some(piece) = iter.next() {
                    if piece.len() == 0 {
                        break;
                    }

                    let mut sum_x = 0 as isize;
                    let mut sum_y = 0 as isize;

                    for &i in piece {
                        sum_x += input.xy[i].0 as isize;
                        sum_y += input.xy[i].1 as isize;
                    }

                    sum_x /= piece.len() as isize;
                    sum_y /= piece.len() as isize;

                    if sum_x != 0 && sum_y != 0 {
                        lines.push((
                            sum_x as i32,
                            sum_y as i32,
                            input.xy[piece[0]].0 + 1,
                            input.xy[piece[0]].1 + 1,
                        ));
                    } else {
                        break;
                    }

                    let (score, _error, (_b, _pieces, not_assigned)) =
                        compute_score(&input, &lines);

                    candi.push((score, lines.clone(), not_assigned.clone()));

                    // println!("score:{}", score);

                    if score > max_score {
                        max_score = score;
                        result = lines.clone();

                        // println!("not assigned max更新:{}", score);
                    }
                }
            }
        }
        // }

        for _ in 0..2 {
            if input.k > current_lines.len() {
                let mut lines = current_lines.clone();

                let modified_point = (
                    rng.gen_range(-NARROW_POINT, NARROW_POINT),
                    rng.gen_range(-NARROW_POINT, NARROW_POINT),
                    rng.gen_range(-NARROW_POINT, NARROW_POINT),
                    rng.gen_range(-NARROW_POINT, NARROW_POINT),
                );

                lines.push(modified_point);

                let (score, _error, (_b, _pieces, not_assigned)) = compute_score(&input, &lines);

                candi.push((score, lines.clone(), not_assigned.clone()));

                // println!("score:{}", score);

                if score > max_score {
                    max_score = score;
                    result = lines.clone();

                    // println!("max更新:{}", score);
                }
            }
        }

        candi.sort_by_key(|(score, _lines, _)| std::cmp::Reverse(score.clone()));

        // eprintln!("queue size {}", priority_queue.len());

        // top 3
        for i in 0..3 {
            priority_queue.push((candi[i].0, candi[i].1.clone(), candi[i].2.clone()));
        }
    }

    output(input.k, &result);
}

pub fn initialize_lines(k: usize) -> Output {
    // let mut rng = rand::thread_rng();

    // let mut result = Vec::new();

    // for _ in 0..10 {
    //     let modified_point = (
    //         rng.gen_range(-POINT_MAX, POINT_MAX),
    //         rng.gen_range(-POINT_MAX, POINT_MAX),
    //         rng.gen_range(-POINT_MAX, POINT_MAX),
    //         rng.gen_range(-POINT_MAX, POINT_MAX),
    //     );

    //     result.push(modified_point);
    // }

    // result

    let mut result_30 = Vec::new();

    result_30.push((-1812, -1984, -9663, 5131));
    result_30.push((-2586, 7859, 7423, -4798));
    result_30.push((193, 8457, -300, 2680));
    result_30.push((-149, -9041, 6425, 84));
    result_30.push((3329, -5601, -2055, -5156));
    result_30.push((4701, 3753, -2852, -2578));
    result_30.push((-1417, -9909, -5593, -5639));
    result_30.push((-7013, 2309, -7622, 6652));
    result_30.push((-5296, -5647, -9646, -1031));
    result_30.push((-1275, 8332, 6134, 1727));
    result_30.push((-6996, 2724, 933, 4067));
    result_30.push((1134, 3022, -9377, 2501));
    result_30.push((6487, -7016, -6730, 4775));
    result_30.push((9141, -7928, 7794, -4177));
    result_30.push((-5164, -3440, -2341, -6820));
    result_30.push((-5940, 2165, -1255, 6089));
    result_30.push((1691, -1616, 347, -8205));
    result_30.push((-7966, -6024, -2883, 5136));
    result_30.push((6899, 3888, 1191, 6179));
    result_30.push((-762, 7446, 2182, 2919));
    result_30.push((-6843, 1720, -8636, -9995));
    result_30.push((-2997, -6378, 4359, -2044));
    result_30.push((-7094, 2392, 2381, 558));
    result_30.push((-3241, 7718, -2179, -1519));
    result_30.push((-358, 247, -547, -5282));
    result_30.push((-376, -1900, 7187, 3663));
    result_30.push((7470, 1828, 4310, -103));
    result_30.push((-2289, 7865, 8712, -1962));
    result_30.push((-6942, -5180, -1909, 326));
    result_30.push((3266, 3617, 3585, 1090));

    result_30

    // let mut result = Vec::new();

    // for i in 0..k {
    //     result.push(result_30[i]);
    // }

    // result
}

pub fn compute_score(
    input: &Input,
    out: &Output,
) -> (i64, String, (Vec<i32>, Vec<Vec<usize>>, Vec<Vec<usize>>)) {
    let mut pieces = vec![(0..input.n).collect::<Vec<_>>()];
    for &(px, py, qx, qy) in out {
        let mut new_pieces = vec![];
        for piece in pieces {
            let mut left = vec![];
            let mut right = vec![];
            for j in piece {
                let (x, y) = input.xy[j];
                let side = (qx - px) as i64 * (y - py) as i64 - (qy - py) as i64 * (x - px) as i64;
                if side > 0 {
                    left.push(j);
                } else if side < 0 {
                    right.push(j);
                }
            }
            if left.len() > 0 {
                new_pieces.push(left);
            }
            if right.len() > 0 {
                new_pieces.push(right);
            }
        }
        pieces = new_pieces;
    }
    let mut b = vec![0; 10];
    let mut not_assigned = Vec::new();
    for piece in &pieces {
        if piece.len() <= 10 {
            b[piece.len() - 1] += 1;
        } else {
            not_assigned.push(piece.clone());
        }
    }
    let mut num = 0;
    let mut den = 0;
    for d in 0..10 {
        num += input.a[d].min(b[d]);
        den += input.a[d];
    }
    let score = (1e6 * num as f64 / den as f64).round() as i64;
    (score, String::new(), (b, pieces, not_assigned))
}

fn input() -> Input {
    use std::io;

    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let a: Vec<i32> = iter.map(|x| x.parse().unwrap()).collect();

    let mut xy: Vec<(i32, i32)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();

        xy.push((x, y));
    }

    Input { n, k, xy, a }
}

fn output(k: usize, result: &Vec<(i32, i32, i32, i32)>) {
    let size = if result.len() <= k { result.len() } else { k };

    println!("{}", size);

    for i in 0..size {
        let p = result[i];

        println!("{} {} {} {}", p.0, p.1, p.2, p.3);
    }
}
