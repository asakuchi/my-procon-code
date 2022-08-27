use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [isize; n],
    }

    let mut result = -1_000_000_000_000_000_000;

    'start_loop: for start in 0..n {
        let mut current = start;

        let mut counter = HashMap::new();
        let mut loop_list = Vec::new();

        let mut score = 0;
        let mut max_score = -1_000_000_000_000_000_000;
        let mut loop_score = 0;

        let mut hands = k;

        loop {
            hands -= 1;

            current = p[current];
            score += c[current];

            max_score = max_score.max(score);

            if hands == 0 {
                result = result.max(max_score);
                continue 'start_loop;
            }

            let count = counter.entry(current).or_insert(0);

            *count += 1;

            if *count == 2 {
                loop_list.push(current);
                loop_score += c[current];
            } else if *count == 3 {
                break;
            }
        }

        hands = k;
        score = 0;

        let loop_count = (hands / loop_list.len()) as isize;

        if loop_count == 0 {
            for i in 0..hands {
                score += c[i];
                max_score = max_score.max(score);
            }
        } else {
            // 1周分に満たない分とは別に1周分を確保する
            score += (loop_count - 1) * loop_score;

            max_score = max_score.max(score);

            hands %= loop_list.len();

            current = start;

            // 1周分に満たない分とは別に1週分もチェックする
            for _ in 0..hands + loop_list.len() {
                current = p[current];
                score += c[current];

                max_score = max_score.max(score);
            }
        }

        result = result.max(max_score);
    }

    println!("{}", result);
}
