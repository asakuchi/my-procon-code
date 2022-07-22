//!
//! 木
//! 2部グラフ
//!

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    let mut visited = vec![false; n];
    let mut colors = vec![0; n];

    rec(n, &list, 0, 1, &mut visited, &mut colors);

    let mut color1_count = 0;
    let mut color2_count = 0;

    for &color in &colors {
        if color == 1 {
            color1_count += 1;
        } else {
            color2_count += 1;
        }
    }

    let answer = colors
        .iter()
        .enumerate()
        .filter(|(_i, &color)| color == if color1_count > color2_count { 1 } else { 2 })
        .take(n / 2)
        .map(|(i, &_color)| (i + 1).to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", answer);
}

fn rec(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    current_color: usize,
    visited: &mut Vec<bool>,
    colors: &mut Vec<usize>,
) {
    colors[current] = current_color;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        rec(
            n,
            list,
            next,
            if current_color == 1 { 2 } else { 1 },
            visited,
            colors,
        );
    }
}
