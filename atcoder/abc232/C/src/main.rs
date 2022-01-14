// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        cd: [(usize, usize); m],
    }

    // ボールの対応表を作成
    let mut ab_matrix = vec![vec![false; n]; n];
    let mut cd_matrix = vec![vec![false; n]; n];

    for (a, b) in ab.iter() {
        ab_matrix[*a - 1][*b - 1] = true;
        ab_matrix[*b - 1][*a - 1] = true;
    }

    for (c, d) in cd.iter() {
        cd_matrix[*c - 1][*d - 1] = true;
        cd_matrix[*d - 1][*c - 1] = true;
    }

    // 紐がつながってないボールの数を比較
    let mut independent_ball_ab = 0;

    for ball in ab_matrix.iter() {
        let mut count = 0;
        for &other in ball {
            if other {
                count += 1;
            }
        }
        if count == 0 {
            independent_ball_ab += 0;
        }
    }

    let mut independent_ball_cd = 0;

    for ball in cd_matrix.iter() {
        let mut count = 0;
        for &other in ball {
            if other {
                count += 1;
            }
        }
        if count == 0 {
            independent_ball_cd += 0;
        }
    }

    if independent_ball_ab != independent_ball_cd {
        println!("No");
        return;
    }

    // 繋がっている紐の本数ごとにボールを数える
    let mut ball_count_per_string_ab = std::collections::HashMap::new();
    let mut ball_count_per_string_cd = std::collections::HashMap::new();

    for ball in ab_matrix.iter() {
        let mut count = 0;
        for &other in ball {
            if other {
                count += 1;
            }
        }

        *ball_count_per_string_ab.entry(count).or_insert(0) += 1;
    }

    for ball in cd_matrix.iter() {
        let mut count = 0;
        for &other in ball {
            if other {
                count += 1;
            }
        }

        *ball_count_per_string_cd.entry(count).or_insert(0) += 1;
    }

    for num in 0..n {
        if ball_count_per_string_ab.entry(num).or_insert(0)
            != ball_count_per_string_cd.entry(num).or_insert(0)
        {
            println!("No");
            return;
        }
    }

    // 同一グループの個数
    let mut ball_count_per_group_ab = Vec::new();
    let mut checked_ab = vec![false; n];

    for i in 0..n {
        if !checked_ab[i] {
            let count = dfs(&ab_matrix, &mut checked_ab, i);
            ball_count_per_group_ab.push(count);
        }
    }

    let mut ball_count_per_group_cd = Vec::new();
    let mut checked_cd = vec![false; n];

    for i in 0..n {
        if !checked_cd[i] {
            let count = dfs(&cd_matrix, &mut checked_cd, i);
            ball_count_per_group_cd.push(count);
        }
    }

    ball_count_per_group_ab.sort();
    ball_count_per_group_cd.sort();

    for (&ab, &cd) in ball_count_per_group_ab
        .iter()
        .zip(ball_count_per_group_cd.iter())
    {
        if ab != cd {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn dfs(matrix: &Vec<Vec<bool>>, checked: &mut Vec<bool>, index: usize) -> usize {
    checked[index] = true;

    let mut count = 1;

    for (j, &other) in matrix[index].iter().enumerate() {
        if other && !checked[j] {
            count += dfs(matrix, checked, j);
        }
    }

    count
}
