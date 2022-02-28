use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

///
/// 最小全域木
/// クラスカルのアルゴリズム
/// Kruskal's Algorithm
///
#[fastout]
fn main() {
    input! {
        n: usize, // 頂点数
        m: usize, // 辺の数
        k: usize,
        mut abc: [(Usize1, Usize1, usize); m], // 辺が結ぶ2点の頂点と重み
    }

    // ------------------------------------

    // 最小全域木の辺の重みの総和
    let mut total_cost = 0;

    let mut size = n;

    // 重みの昇順にソート
    abc.sort_by_key(|x| x.2);

    let mut set = UnionFind::new(n);

    for i in 0..m {
        if size == k {
            break;
        }

        if !set.equiv(abc[i].0, abc[i].1) {
            total_cost += abc[i].2;
            set.union(abc[i].0, abc[i].1);

            size -= 1;
        }
    }

    println!("{}", total_cost);
}
