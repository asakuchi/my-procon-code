use proconio::fastout;
use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    /// 未訪問
    White,
    /// 候補
    Gray,
    /// 確定
    Black,
}

///
/// 最小全域木
/// プリムのアルゴリズム
/// Prim's Algorithm
///
fn main() {
    input! {
        n: usize, // 頂点数
        a: [[isize; n]; n], // 隣接行列
    }

    // ------------------------------------

    // 訪問状態
    let mut color = vec![Color::White; n];
    // 重みが最小の辺の重み
    let mut d = vec![std::isize::MAX; n];
    // 頂点の親
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

    // 最小全域木の辺の重みの総和
    let mut sum = 0;

    for i in 0..n {
        if p[i] != -1 {
            sum += a[i][p[i] as usize];
        }
    }

    println!("{}", sum);
}
