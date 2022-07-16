// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

///
/// 第8章 円の国家群
///
fn main() {
    input! {
        n: u32, // 要素数
        roads: [Chars; n],
    }

    let result = count_paths(&roads);

    println!("{}", result);
}

fn count_paths(roads: &Vec<Vec<char>>) -> usize {
    let mut group = 0;
    let mut free = 0;

    let mut connect = vec![0; roads.len()];
    let mut sum: i64 = 1;

    let mut visited = vec![false; roads.len()];

    // 枝分かれしている都市の確認
    // 通らなければいけない道路の数を都市ごとにカウント
    for (i, road) in roads.iter().enumerate() {
        let mut count = 0;

        for &point in road.iter() {
            if point == 'Y' {
                count += 1;
            }
        }

        if count >= 3 {
            return 0;
        }

        connect[i] = count;
    }

    for i in 0..connect.len() {
        // 自由に通れる都市をカウント
        if connect[i] == 0 {
            visited[i] = true;
            free += 1;
        // 通らなければいけない道路は端の都市（= 道路の数が1）からカウント
        } else if connect[i] == 1 && !visited[i] {
            group += 1;
            dfs(&roads, &mut visited, i);
        }
    }

    for i in 0..visited.len() {
        if !visited[i] {
            return 0;
        }
    }

    for i in 0..group + free {
        sum = sum * (i + 1) % 1000000007;
    }

    for _ in 0..group {
        sum = sum * 2 % 1000000007;
    }

    sum as usize
}

fn dfs(roads: &Vec<Vec<char>>, visited: &mut Vec<bool>, city: usize) {
    visited[city] = true;

    for i in 0..roads.len() {
        if roads[city][i] == 'Y' && !visited[i] {
            dfs(roads, visited, i);
        }
    }
}
