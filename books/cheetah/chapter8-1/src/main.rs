// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 第8章 カラフルボックス＆ボール
///
fn main() {
    input! {
        num_red: usize,
        num_blue: usize,
        only_red: isize,
        only_blue: isize,
        both_colors: isize,
    }

    let all_same_score = only_red * num_red as isize + only_blue * num_blue as isize;

    let mut remaining_red = num_red;
    let mut remaining_blue = num_blue;
    let mut both_num = 0;

    let mut max_score = all_same_score;

    // println!("same_score:{}", all_same_score);

    while remaining_red != 0 && remaining_blue != 0 {
        remaining_red -= 1;
        remaining_blue -= 1;
        both_num += 2;

        let score = only_red * remaining_red as isize
            + only_blue * remaining_blue as isize
            + both_colors * both_num;

        // println!("score:{}", score);

        max_score = std::cmp::max(max_score, score);
    }

    println!("{}", max_score);
}
