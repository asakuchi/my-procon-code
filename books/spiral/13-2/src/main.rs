// -*- coding:utf-8-unix -*-

use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

///
/// 最小全域木
///
fn main() {
    input! {
        n: usize,
        a: [[isize; n]; n],
    }

    let mut color = vec![Color::White; n];
    // let mut m = vec![vec![std::isize::MAX; n]; n];
    let mut d = vec![std::isize::MAX; n];
    let mut p = vec![-1; n];

    d[0] = 0;
    p[0] = -1;

    loop {
        let mut min_cost = std::isize::MAX;
        let mut u = std::usize::MAX;

        for i in 0..n {
            if color[i] != Color::Black && d[i] < min_cost {
                min_cost = d[i];
                u = i;
            }
        }

        if min_cost == std::isize::MAX {
            break;
        }

        color[u] = Color::Black;

        for v in 0..n {
            if color[v] != Color::Black && a[u][v] != -1 {
                if a[u][v] < d[v] {
                    d[v] = a[u][v];
                    p[v] = u as isize;
                    color[v] = Color::Gray;
                }
            }
        }
    }

    println!("{:?}", p);

    let mut sum = 0;

    for i in 0..n {
        if p[i] != -1 {
            sum += a[i][p[i] as usize];
        }
    }

    println!("{:?}", sum);

    // println!("{}", result);
}
